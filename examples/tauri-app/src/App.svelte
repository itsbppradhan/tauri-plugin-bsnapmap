<script>
  import Greet from './lib/Greet.svelte'
  import { ping, trackMousePosition } from 'tauri-plugin-bsnapmap-api'
  import { getCurrentWindow } from '@tauri-apps/api/window'
  import { onMount } from 'svelte';

	let response = ''
	let mouseX = 0;
	let mouseY = 0;

	function updateResponse(returnValue) {
		response += `[${new Date().toLocaleTimeString()}] ` + (typeof returnValue === 'string' ? returnValue : JSON.stringify(returnValue)) + '<br>'
	}

	function _ping() {
		ping("Pong!").then(updateResponse).catch(updateResponse)
	}

  onMount(() => {
  // when using `"withGlobalTauri": true`, you may use
  //const { getCurrentWindow } = window.__TAURI__.window;

  const appWindow = getCurrentWindow();

  document
    .getElementById('titlebar-minimize')
    ?.addEventListener('click', () => appWindow.minimize());
  document
    .getElementById('titlebar-maximize')
    ?.addEventListener('click', () => appWindow.toggleMaximize());
  document
    .getElementById('titlebar-close')
    ?.addEventListener('click', () => appWindow.close());

    const cleanup = trackMousePosition(async (pos) => {
      try {
        mouseX = pos.x;
        mouseY = pos.y;
        console.log('Mouse position:', pos); // Debug log
      } catch (e) {
        console.error('Error tracking mouse:', e);
      }
    });

    return () => cleanup();
  });
</script>

<main class="container">
  <h1>Welcome to Tauri!</h1>

  <div class="row">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>

  <p>
    Click on the Tauri, Vite, and Svelte logos to learn more.
  </p>

  <div class="row">
    <Greet />
  </div>

  <div>
    <button on:click="{_ping}">Ping</button>
    <div>{@html response}</div>
  </div>

  <div>
    Mouse Position: {mouseX}, {mouseY}
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
