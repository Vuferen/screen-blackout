<script>
	import { appWindow } from '@tauri-apps/api/window';
	import { listen, emit } from '@tauri-apps/api/event';
  	import { onMount } from 'svelte';

	let showContent = false;
	let timer;

	// function close() {
	// 	emit("close");
	// 	appWindow.close();
	// }
	function hide() {
		emit("hide");
		appWindow.hide();
	}
	
	onMount(async () => {
		await listen("show", (event) => {
			appWindow.show();
		});
		await listen("hide", (event) => {
			appWindow.hide();
		});
		await listen("close", (event) => {
			appWindow.close();
		});
		//appWindow.show(); // Change this such that the blackout windows are loaded but hidden from the start and the blackout button simply shows the windows.
	});


	function showForTime() {
		showContent = true;
		clearTimeout(timer);
		timer = setTimeout(() => {
			showContent = false;
		}, 1000);
	}

</script>

<main style="cursor: {!showContent ? 'none': 'default'};" class=" bg-black w-screen h-screen grid place-items-center" on:mousemove={showForTime}>
	{#if showContent}
		<div class="w-fit h-fit">
			<button class=" bg-slate-600" on:click={hide}>Close</button>
		</div>
	{/if}
</main>