import { invoke } from '@tauri-apps/api/tauri'
import { appWindow, WebviewWindow } from '@tauri-apps/api/window'

const w: WebviewWindow = appWindow

export async function spawn() {
  
  await invoke('plugin:mqtt|new_client')

}

export async function publish() {
  
	await invoke('plugin:mqtt|publish')
  
  }
