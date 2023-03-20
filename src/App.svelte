<script lang="ts">
	import { emit, listen } from "@tauri-apps/api/event";
	import {
		availableMonitors,
		currentMonitor,
		WebviewWindow,
		appWindow,
	} from "@tauri-apps/api/window";
	import { onMount } from "svelte";
	import { register } from '@tauri-apps/api/globalShortcut';

	const monitors = availableMonitors();
	let isInBlackout = false;

	onMount(async () => {
		// Receive events from taskbar
		await listen("blackout", (event) => {
			showBlackouts();
		});
		await listen("stop-blackout", (event) => {
			hideBlackouts();
		});

		appWindow.setDecorations(false);

		document
			.getElementById("titlebar-minimize")
			.addEventListener("click", () => appWindow.minimize());
			
		document
			.getElementById("titlebar-maximize")
			.addEventListener("click", () => appWindow.toggleMaximize());
		document
			.getElementById("titlebar-close")
			.addEventListener("click", () => appWindow.hide());
		
		await register('CommandOrControl+Alt+B', () => {
			isInBlackout ? hideBlackouts() : showBlackouts();
		});

		spawnBlackoutWindows();
	});

	async function spawnBlackoutWindows() {
		let monitor = await currentMonitor();

		(await monitors).forEach((screen) => {
			if (screen.name != monitor.name) {
				let windowLabel = screen.name.slice(screen.name.indexOf("D"));
				windowLabel = windowLabel.replace("1", "One");
				console.log(windowLabel);
				console.log(screen.position);
				const webview = new WebviewWindow(windowLabel, {
					url: "blackout.html",
					x: screen.position.x,
					y: screen.position.y - 1,
					width: screen.size.width,
					height: screen.size.height + 1,
					decorations: false,
					alwaysOnTop: true,
					resizable: false,
					visible: false,
				});

				// webview.once("tauri://created", function () {
				// 	// webview window successfully created
				// 	console.log("Yay");
				// });
				// webview.once("tauri://error", function (e) {
				// 	console.log(e);
				// });
			}
		});
	}
	function closeBlackouts() {
		emit("close");
	}
	function hideBlackouts() {
		emit("hide");
		isInBlackout = false;
	}
	function showBlackouts() {
		emit("show");
		isInBlackout = true;
	}

</script>

<main class="container">
	<!-- <h1>Lorem ipsum</h1> -->
	<!-- <div class="mt-5">
	</div> -->
	{#if isInBlackout}
		<button class="stop" on:click={() => hideBlackouts()}>Stop Blackout</button>
	{:else}
		<button class="start" on:click={() => showBlackouts()}>Start Blackout</button>
	{/if}
</main>

<style>
	h1 {
		font-size: 2.7em;
		margin: 1rem;
		margin-top: 2rem;
	}
	.logo.vite:hover {
		filter: drop-shadow(0 0 2em #747bff);
	}

	.logo.svelte:hover {
		filter: drop-shadow(0 0 2em #ff3e00);
	}
	button{
		border-radius: 0;
		height: 170px;
		font-weight: bold;
		font-size: 2em;
		border: none;
	}
	button:hover{
		outline: none;
		border:none;
	}
	button.start{
		background-color: rgb(36, 40, 88);
	}
	button.start:hover {
		background-color: black;
	}
	button.stop{
		background-color: rgb(119, 32, 32);
	}
	button.stop:hover{
		background-color: rgb(139, 18, 18);
	}
	.container{
		height: 100%;
	}
</style>
