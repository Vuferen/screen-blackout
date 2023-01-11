<script>
	import { appWindow } from '@tauri-apps/api/window';
	import { listen, emit } from '@tauri-apps/api/event';
  	import { onMount } from 'svelte';

	let showContent = false;
	let timer;

	function close() {
		emit("close");
		appWindow.close();
	}
	
	onMount(async () => {
		const unlisten = await listen("close", (event) => {
			unlisten();
			appWindow.close();
		});
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
			<button class=" bg-slate-600" on:click={close}>Close</button>
		</div>
	{/if}
</main>