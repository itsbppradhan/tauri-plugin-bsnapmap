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

export async function getLParamMousePosition(): Promise<MousePosition> {
  return invoke('plugin:bsnapmap|get_lparam_mouse_position');
}

export async function getMappedMousePosition(): Promise<MousePosition> {
  return invoke('plugin:bsnapmap|get_mapped_mouse_position');
}

export function trackAllMousePositions(callback: (positions: {
  tauri: MousePosition, 
  win32: MousePosition,
  lparam: MousePosition,
  mapped: MousePosition
}) => void, interval = 100) {
  const tracker = setInterval(async () => {
    const tauri = await getMousePosition();
    const win32 = await getWin32MousePosition();
    const lparam = await getLParamMousePosition();
    const mapped = await getMappedMousePosition();
    callback({ tauri, win32, lparam, mapped });
  }, interval);
  return () => clearInterval(tracker);
}

export interface ButtonRect {
  left: number;
  top: number;
  right: number;
  bottom: number;
}

export async function setMaximizeButtonRect(): Promise<void> {
  const button = document.querySelector('[data-tauri-maximize-region]');
  if (!button) return;
  
  // Get DPI scale factor
  const dpiScale = window.devicePixelRatio;
  
  const rect = button.getBoundingClientRect();
  const buttonRect: ButtonRect = {
    left: Math.round(rect.left * dpiScale),
    top: Math.round(rect.top * dpiScale),
    right: Math.round(rect.right * dpiScale),
    bottom: Math.round(rect.bottom * dpiScale)
  };
  
  return invoke('plugin:bsnapmap|set_maximize_button_rect', { rect: buttonRect });
}

export async function isOverMaximizeButton(): Promise<boolean> {
  return invoke('plugin:bsnapmap|is_over_maximize_button');
}

export function trackMaximizeButtonHover(callback: (isOver: boolean) => void, interval = 50) {
  const tracker = setInterval(async () => {
    const isOver = await isOverMaximizeButton();
    callback(isOver);
  }, interval);
  return () => clearInterval(tracker);
}