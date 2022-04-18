<script lang="ts">
	import { spawn } from 'tauri-plugin-mqtt'
	// import { invoke } from '@tauri-apps/api/tauri'

	function makeClient(){
		spawn();
	}


</script>

<div>
	<button on:click="{makeClient}">Watch</button>
	
</div>
