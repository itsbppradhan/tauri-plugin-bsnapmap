use tauri::{AppHandle, command, Runtime, Window};
use crate::models::*;
use crate::Result;
use crate::BsnapmapExt;

#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::{POINT, LPARAM, HWND},
    UI::WindowsAndMessaging::{GetCursorPos, HWND_DESKTOP},
    Graphics::Gdi::MapWindowPoints,
};

#[cfg(target_os = "windows")]
use windows::Win32::Graphics::Gdi::ScreenToClient;

#[command(rename_all = "snake_case")]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.bsnapmap().ping(payload)
}

#[cfg(target_os = "windows")]
#[command(rename_all = "snake_case", permission = "bsnapmap:allow-get-mouse-position")]
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
#[command(rename_all = "snake_case", permission = "bsnapmap:allow-get-mouse-position")]
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
#[command(rename_all = "snake_case", permission = "bsnapmap:allow-get-win32-mouse-position")]
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
#[command(rename_all = "snake_case", permission = "bsnapmap:allow-get-lparam-mouse-position")]
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

#[cfg(target_os = "windows")]
#[command(rename_all = "snake_case", permission = "bsnapmap:allow-get-mapped-mouse-position")]
pub(crate) async fn get_mapped_mouse_position<R: Runtime>(
    window: Window<R>,
) -> Result<MousePosition> {
    let mut point = POINT { x: 0, y: 0 };
    unsafe {
        // First get cursor position in screen coordinates
        GetCursorPos(&mut point)?;
        
        // Get the HWND and convert it properly for the Windows API
        let raw_handle = window.hwnd()?;
        let hwnd = HWND(raw_handle.0 as isize);
        
        // Map from screen coordinates to window client coordinates
        ScreenToClient(hwnd, &mut point);
    }
    
    Ok(MousePosition {
        x: point.x as f64,
        y: point.y as f64,
    })
}

#[cfg(target_os = "windows")]
#[command(rename_all = "snake_case", permission = "bsnapmap:allow-map-window-points")]
pub(crate) async fn map_window_points<R: Runtime>(
    window_from: Option<Window<R>>,
    window_to: Option<Window<R>>,
    points: Vec<MousePosition>,
) -> Result<Vec<MousePosition>> {
    let mut win_points: Vec<POINT> = points.iter()
        .map(|p| POINT { 
            x: p.x as i32, 
            y: p.y as i32 
        })
        .collect();

    unsafe {
        let hwnd_from = if let Some(w) = window_from {
            HWND(w.hwnd()?.0 as isize)
        } else {
            HWND_DESKTOP
        };

        let hwnd_to = if let Some(w) = window_to {
            HWND(w.hwnd()?.0 as isize)
        } else {
            HWND_DESKTOP
        };

        MapWindowPoints(
            hwnd_from,
            hwnd_to,
            &mut win_points,
        );
    }

    Ok(win_points.into_iter()
        .map(|p| MousePosition {
            x: p.x as f64,
            y: p.y as f64,
        })
        .collect())
}
