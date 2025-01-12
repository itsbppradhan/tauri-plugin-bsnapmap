mod commands;
mod models;
mod desktop;

pub use desktop::{Bsnapmap, BsnapmapExt};
pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error + Send + Sync>;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};
use anyhow::anyhow;

use windows::Win32::{
    Foundation::{HWND, WPARAM, LPARAM, LRESULT, POINT},
    UI::WindowsAndMessaging::{
        WM_NCHITTEST, DefWindowProcW, HTMAXBUTTON, SetWindowLongPtrW, GWLP_WNDPROC
    },
    Graphics::Gdi::ScreenToClient,
};

use crate::commands::{MAXIMIZE_BUTTON_RECT, get_x_lparam, get_y_lparam};

#[cfg(target_os = "windows")]
pub(crate) unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_NCHITTEST => {
            if let Some(rect) = &*MAXIMIZE_BUTTON_RECT.lock().unwrap() {
                let x = get_x_lparam(lparam);
                let y = get_y_lparam(lparam);
                let mut point = POINT { x, y };
                ScreenToClient(hwnd, &mut point);
                
                if point.x >= rect.left 
                    && point.x <= rect.right 
                    && point.y >= rect.top 
                    && point.y <= rect.bottom {
                    return LRESULT(HTMAXBUTTON as isize);
                }
            }
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam)
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("bsnapmap")
        .invoke_handler(tauri::generate_handler![
            commands::ping,
            commands::get_mouse_position,
            commands::get_win32_mouse_position,
            commands::get_lparam_mouse_position,
            commands::get_mapped_mouse_position,
            commands::map_window_points,
            commands::set_maximize_button_rect,
            commands::is_over_maximize_button,
        ])
        .setup(|app, api| {
            #[cfg(target_os = "windows")]
            if let Some(window) = app.get_webview_window("main") {
                let handle = window.hwnd().unwrap();
                let hwnd = HWND(handle.0 as isize);
                unsafe {
                    SetWindowLongPtrW(hwnd, GWLP_WNDPROC, wndproc as isize);
                }
            }

            let bsnapmap = desktop::init(app, api).map_err(|e| anyhow!(e.to_string()))?;
            app.manage(bsnapmap);
            Ok(())
        })
        .build()
}
