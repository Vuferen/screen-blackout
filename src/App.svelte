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
	import { Store } from "tauri-plugin-store-api";

	const store = new Store(".settings.dat");

	let monitors;
	let isInBlackout = false;

	let hideOnStartup = true;

	onMount(async () => {
		console.log(hideOnStartup);
		hideOnStartup = await store.get("hide-on-startup");
		console.log(hideOnStartup);

		if (!hideOnStartup) {
			appWindow.show();
		}

		monitors = await availableMonitors();
		// Receive events from taskbar
		await listen("blackout", (event) => {
			showBlackouts();
		});
		await listen("stop-blackout", (event) => {
			hideBlackouts();
		});

		// Receive events from other windows
		await listen("hide", (event) => {
			isInBlackout = false;
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

		monitors.forEach((screen) => {
			if (screen.name != monitor.name) {
				let windowLabel = screen.name.slice(screen.name.indexOf("D"));
				windowLabel = windowLabel.replace(/\d+/g, replacer);
				new WebviewWindow(windowLabel, {
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
			}
		});
	}

	async function toggleHideOnStartup() {
		hideOnStartup = !hideOnStartup;
		await store.set("hide-on-startup", hideOnStartup);
		await store.save();
	}

	// Function to replace 0-9 with A-J
	function replacer(match) {
		return match
			.split('')     // convert string to array of characters
			.map(Number)   // parse characters as numbers
			.map(n => (n || 10) + 64)   // convert to char code, correcting for J
			.map(c => String.fromCharCode(c))   // convert char codes to strings
			.join('');     // join values together
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
	{#if isInBlackout}
		<button class="stop" on:click={() => hideBlackouts()}>Stop Blackout</button>
	{:else}
		<button class="start" on:click={() => showBlackouts()}>Start Blackout</button>
	{/if}
	<label class="setting-line" for="hide-on-startup">Hide on startup: <input type="checkbox" name="hide-on-startup" checked={hideOnStartup} on:click={toggleHideOnStartup}></label>
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
		background-color: rgb(28, 40, 73)
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
	.setting-line {
		align-self: baseline;
		padding: 0.3em 1em;
	}
</style>
