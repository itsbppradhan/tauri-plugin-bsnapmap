use tauri::{AppHandle, command, Runtime, Window};
use crate::models::*;
use crate::Result;
use crate::BsnapmapExt;

#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::{POINT, LPARAM},
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

#[cfg(target_os = "windows")]
fn get_x_lparam(lparam: LPARAM) -> i32 {
    (lparam.0 & 0xFFFF) as i32
}

#[cfg(target_os = "windows")]
fn get_y_lparam(lparam: LPARAM) -> i32 {
    ((lparam.0 >> 16) & 0xFFFF) as i32
}

#[cfg(target_os = "windows")]
#[command]
pub(crate) async fn get_lparam_mouse_position<R: Runtime>(
    _window: Window<R>,
) -> Result<MousePosition> {
    let mut point = POINT { x: 0, y: 0 };
    unsafe {
        GetCursorPos(&mut point)?;
    }
    // Pack coordinates into LPARAM format
    let lparam = LPARAM(((point.y as i32) << 16 | (point.x as i32 & 0xFFFF)) as isize);
    // Extract using our helper functions
    let x = get_x_lparam(lparam);
    let y = get_y_lparam(lparam);
    
    Ok(MousePosition {
        x: x as f64,
        y: y as f64,
    })
}
