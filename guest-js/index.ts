import { invoke } from '@tauri-apps/api/core'

export function ping(value: string): Promise<string | null> {
  return invoke<{value?: string}>('plugin:bsnapmap|ping', {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null));
}

export interface MousePosition {
  x: number;
  y: number;
}

export async function getMousePosition(): Promise<MousePosition> {
  return invoke('plugin:bsnapmap|get_mouse_position');
}

export function trackMousePosition(callback: (position: MousePosition) => void, interval = 100) {
  const tracker = setInterval(async () => {
    callback(await getMousePosition());
  }, interval);
  return () => clearInterval(tracker);
}