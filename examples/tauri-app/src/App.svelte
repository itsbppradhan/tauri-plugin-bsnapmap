<script>
  import Greet from './lib/Greet.svelte'
  import { ping, trackMousePosition, trackBothMousePositions, trackAllMousePositions, setMaximizeButtonRect } from 'tauri-plugin-bsnapmap-api'
  import { getCurrentWindow } from '@tauri-apps/api/window'
  import { onMount } from 'svelte';

	let response = ''
	let mouseX = 0;
	let mouseY = 0;
	let win32X = 0;
	let win32Y = 0;
	let lparamX = 0;
	let lparamY = 0;
	let mappedX = 0;
	let mappedY = 0;
	let buttonLeft = 0;
	let buttonTop = 0;
	let buttonRight = 0;
	let buttonBottom = 0;

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

    const cleanup = trackAllMousePositions(async (pos) => {
      try {
        mouseX = pos.tauri.x;
        mouseY = pos.tauri.y;
        win32X = pos.win32.x;
        win32Y = pos.win32.y;
        lparamX = pos.lparam.x;
        lparamY = pos.lparam.y;
        mappedX = pos.mapped.x;
        mappedY = pos.mapped.y;
        console.log('Mouse positions:', pos);
      } catch (e) {
        console.error('Error tracking mouse:', e);
      }
    });

    const updateButtonRect = () => {
      const button = document.querySelector('[data-tauri-maximize-region]');
      if (button) {
        const dpiScale = window.devicePixelRatio;
        const rect = button.getBoundingClientRect();
        buttonLeft = Math.round(rect.left * dpiScale);
        buttonTop = Math.round(rect.top * dpiScale);
        buttonRight = Math.round(rect.right * dpiScale);
        buttonBottom = Math.round(rect.bottom * dpiScale);
      }
    };

    updateButtonRect();
    window.addEventListener('resize', updateButtonRect);

    window.matchMedia('(resolution: 1dppx)').addListener(updateButtonRect);
    appWindow.listen('tauri://move', updateButtonRect);

    return () => {
      cleanup();
      window.removeEventListener('resize', updateButtonRect);
    };
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
    Tauri Mouse Position: {mouseX}, {mouseY}
    <br/>
    Win32 Mouse Position: {win32X}, {win32Y}
    <br/>
    LParam Mouse Position: {lparamX}, {lparamY}
    <br/>
    Mapped Window Position: {mappedX}, {mappedY}
  </div>

  <div>
    Maximize Button Coordinates:
    <br/>
    Left: {buttonLeft}, Top: {buttonTop}
    <br/>
    Right: {buttonRight}, Bottom: {buttonBottom}
    <br/>
    Width: {buttonRight - buttonLeft}, Height: {buttonBottom - buttonTop}
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
