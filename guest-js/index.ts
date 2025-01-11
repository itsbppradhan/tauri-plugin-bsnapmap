import { invoke } from '@tauri-apps/api/core'

export function ping(value: string): Promise<string | null> {
  return invoke<{value?: string}>('plugin:bsnapmap|ping', {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null));
}
