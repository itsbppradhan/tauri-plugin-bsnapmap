use tauri::{AppHandle, command, Runtime, Window};
use crate::models::*;
use crate::Result;
use crate::BsnapmapExt;

#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::POINT,
    UI::WindowsAndMessaging::GetCursorPos,
};

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.bsnapmap().ping(payload)
}

#[cfg(target_os = "windows")]
#[command]
pub(crate) async fn get_mouse_position<R: Runtime>(
    _window: Window<R>,
) -> Result<MousePosition> {
    let mut point = POINT { x: 0, y: 0 };
    unsafe {
        GetCursorPos(&mut point)?;
    }
    Ok(MousePosition {
        x: point.x as f64,
        y: point.y as f64,
    })
}

#[cfg(not(target_os = "windows"))]
#[command]
pub(crate) async fn get_mouse_position<R: Runtime>(
    window: Window<R>,
) -> Result<MousePosition> {
    let position = window.cursor_position()?;
    Ok(MousePosition {
        x: position.x,
        y: position.y,
    })
}

#[cfg(target_os = "windows")]
#[command]
pub(crate) async fn get_win32_mouse_position<R: Runtime>(
    _window: Window<R>,
) -> Result<MousePosition> {
    let mut point = POINT { x: 0, y: 0 };
    unsafe {
        GetCursorPos(&mut point)?;
    }
    Ok(MousePosition {
        x: point.x as f64,
        y: point.y as f64,
    })
}
