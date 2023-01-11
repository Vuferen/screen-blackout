<script>
	import { appWindow } from '@tauri-apps/api/window';
	function close() {
		appWindow.close();
	}
</script>

<main class=" group/window bg-black w-screen h-screen grid place-items-center">
	<div class="w-fit h-fit group-hover/window:opacity-100 opacity-0">
		<button class=" bg-slate-600" on:click={close}>Close</button>
	</div>
</main>