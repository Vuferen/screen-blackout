<script lang="ts">
	import { emit } from '@tauri-apps/api/event';
	import { availableMonitors, currentMonitor, WebviewWindow, appWindow } from '@tauri-apps/api/window';
	const monitors = availableMonitors();
  
  	let debugText = "Not pressed";

  	async function blackout() {
		let monitor = await currentMonitor();
		console.log(monitor);
		console.log(await monitors);

		(await monitors).forEach(screen => {
			if (screen.name != monitor.name) {
				let windowLabel = screen.name.slice(screen.name.indexOf('D'));
				windowLabel = windowLabel.replace("1", "One");
				console.log(windowLabel);
				console.log(screen.position);
				const webview = new WebviewWindow(windowLabel, {
					url: 'blackout.html',
					x: screen.position.x,
					y: screen.position.y-1,
					width: screen.size.width,
					height: screen.size.height+1,
					decorations: false,
					alwaysOnTop: true,
					resizable: false,
				});
				webview.once('tauri://created', function () {
				// webview window successfully created
					console.log("Yay")
				});
				webview.once('tauri://error', function (e) {
					console.log(e);
				});

			}
    	});

    	debugText = "Pressed";
  	}
  	function closeBlackouts() {
		emit("close");
	}
</script>

<main class="container">
  <span>{debugText}</span>
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