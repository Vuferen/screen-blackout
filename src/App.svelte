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
	<div class="mt-5">
		<button on:click={() => showBlackouts()}>Start Blackout</button>
		<button on:click={() => hideBlackouts()}>Stop Blackout</button>
	</div>
</main>

<style>

	.logo.vite:hover {
		filter: drop-shadow(0 0 2em #747bff);
	}

	.logo.svelte:hover {
		filter: drop-shadow(0 0 2em #ff3e00);
	}
</style>
