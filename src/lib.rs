use serde::{Deserialize, Serialize};
use std::error;
use std::sync::Mutex;
use std::time::Duration;
use tauri::{
	command,
	plugin::{Builder, TauriPlugin},
	AppHandle, Manager, Runtime, State,
};

#[cfg(feature = "broker")]
pub mod broker;

#[cfg(feature = "client")]
use rumqttc::{self, AsyncClient, EventLoop, MqttOptions, QoS};

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Unable to connect client")]
	ClientError(#[from] rumqttc::ClientError),
	#[error("Invocation error")]
	InvokeError(),
}

#[cfg(feature = "client")]
struct Client {
	pub name: String,
	pub client: AsyncClient,
	loop_handle: Option<tauri::async_runtime::JoinHandle<()>>,
}

impl Client {
	async fn spawn(name: String, server: String) -> Result<Client, Error> {
		let mut mqttoptions = MqttOptions::new(&name, &server, 1883);
		mqttoptions.set_keep_alive(Duration::from_secs(5));
		println!("Doing the thing");
		let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
		let loop_handle = Some(tauri::async_runtime::spawn(async move {
			loop {
				//Maybe a mpsc channel here passed in as arg that we send events to a single reciever for callbacks? break loop on error.
				let notification = eventloop.poll().await.unwrap();
				println!("{:?}", notification);
				//   recv_mq(notification).await;
			}
		}));

		Ok(Client {
			name,
			client,
			loop_handle,
		})
	}

	// 	async fn subscribe(topic:String) -> Result<Error> {
	// 		self.client.subscribe("hello/world", QoS::AtMostOnce)
	//         .await?
	// 	}
}

#[command]
fn publish(state: tauri::State<'_, MyState>) -> Result<(), Error> {
	match state.inner().clients.lock().unwrap()[0]
		.client
		.clone()
		.try_publish("taurimqtt", QoS::ExactlyOnce, false, "Testing".as_bytes())
	{
		Ok(_) => Ok(()),
		rumqttc::ClientError(e) => Err(Error::ClientError(e)),
		tauri::InvokeError(e) => Err(Error::InvokeError()),
		_ => Err(()),
	}
}

#[command]
async fn new_client(state: tauri::State<'_, MyState>) {
	if let mut store = state.inner() {
		let client = Client::spawn(
			String::from("rumqtt-tauri-async"),
			String::from("test.mosquitto.org"),
		)
		.await;

		store.clients.lock().unwrap().push(client.unwrap());
	}
}

#[derive(Default)]
pub struct MyState {
	pub clients: Mutex<Vec<Client>>,
}

impl MyState {
	fn add_client(mut self, client: Client) {
		self.clients.lock().unwrap().push(client);
	}
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
	Builder::new("mqtt")
		.invoke_handler(tauri::generate_handler![new_client, publish])
		.setup(|app_handle| {
			// setup plugin specific state here
			app_handle.manage(MyState::default());
			Ok(())
		})
		.build()
}
