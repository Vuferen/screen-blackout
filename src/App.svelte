<script lang="ts">
	import { emit, listen } from "@tauri-apps/api/event";
	import {
		availableMonitors,
		currentMonitor,
		WebviewWindow,
		appWindow,
	} from "@tauri-apps/api/window";
	import { onMount } from "svelte";
	const monitors = availableMonitors();

	onMount(async () => {
		const unlisten = await listen("blackout", (event) => {
			blackout();
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
	});

	async function blackout() {
		let monitor = await currentMonitor();
		console.log(monitor);
		console.log(await monitors);

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

				webview.once("tauri://created", function () {
					// webview window successfully created
					console.log("Yay");
				});
				webview.once("tauri://error", function (e) {
					console.log(e);
				});
			}
		});
	}
	function closeBlackouts() {
		emit("close");
	}
</script>

<main class="container">
	<!-- <div data-tauri-drag-region class="titlebar">
		<div class="titlebar-button" id="titlebar-minimize">
			<img
				src="https://api.iconify.design/mdi:window-minimize.svg"
				alt="minimize"
			/>
		</div>
		<div class="titlebar-button" id="titlebar-maximize">
			<img
				src="https://api.iconify.design/mdi:window-maximize.svg"
				alt="maximize"
			/>
		</div>
		<div class="titlebar-button" id="titlebar-close">
			<img src="https://api.iconify.design/mdi:close.svg" alt="close" />
		</div>
	</div> -->

	<div class="mt-5">
		<button on:click={() => blackout()}>Blackout</button>
		<button on:click={() => closeBlackouts()}>Close</button>
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
