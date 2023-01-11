<script lang="ts">
  import { availableMonitors, currentMonitor, WebviewWindow } from '@tauri-apps/api/window';
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
				y: screen.position.y,
				width: screen.size.width,
				height: screen.size.height,
				decorations: false,
				alwaysOnTop: true,
				resizable: false,
				focus: false
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
</script>

<main class="container">
  <span>{debugText}</span>
  <div class="mt-5">
    <button on:click={() => blackout()}>Blackout</button>
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