import { invoke } from '@tauri-apps/api/tauri';
import '@tauri-apps/api/window';

async function spawn() {
    await invoke('plugin:mqtt|new_client');
}

async function publish() {
    await invoke('plugin:mqtt|publish');
}

export { spawn, publish };
