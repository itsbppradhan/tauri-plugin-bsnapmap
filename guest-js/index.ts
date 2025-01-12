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

export async function getWin32MousePosition(): Promise<MousePosition> {
  return invoke('plugin:bsnapmap|get_win32_mouse_position');
}

export function trackBothMousePositions(callback: (positions: {tauri: MousePosition, win32: MousePosition}) => void, interval = 100) {
  const tracker = setInterval(async () => {
    const tauri = await getMousePosition();
    const win32 = await getWin32MousePosition();
    callback({ tauri, win32 });
  }, interval);
  return () => clearInterval(tracker);
}