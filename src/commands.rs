use tauri::{AppHandle, command, Runtime, Window, Manager};
use crate::models::*;
use crate::BsnapmapExt;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use anyhow::anyhow;

#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::{POINT, LPARAM, HWND},
    UI::WindowsAndMessaging::{GetCursorPos, HWND_DESKTOP},
    Graphics::Gdi::MapWindowPoints,
};

#[cfg(target_os = "windows")]
use windows::Win32::Graphics::Gdi::ScreenToClient;

pub(crate) static MAXIMIZE_BUTTON_RECT: Lazy<Mutex<Option<ButtonRect>>> = Lazy::new(|| Mutex::new(None));

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse, tauri::Error> {
    app.bsnapmap().ping(payload).map_err(|e| anyhow!("{}", e).into())
}

#[cfg(target_os = "windows")]
#[command(rename_all = "snake_case")]
pub(crate) async fn get_mouse_position<R: Runtime>(
    _window: Window<R>,
) -> Result<MousePosition, tauri::Error> {
    let mut point = POINT { x: 0, y: 0 };
    unsafe {
        GetCursorPos(&mut point).map_err(|e| anyhow!("{}", e))?;
    }
    Ok(MousePosition {
        x: point.x as f64,
        y: point.y as f64,
    })
}

#[cfg(not(target_os = "windows"))]
#[command(rename_all = "snake_case")]
pub(crate) async fn get_mouse_position<R: Runtime>(
    window: Window<R>,
) -> Result<MousePosition, tauri::Error> {
    let position = window.cursor_position().map_err(|e| anyhow!("{}", e))?;
    Ok(MousePosition {
        x: position.x,
        y: position.y,
    })
}

#[cfg(target_os = "windows")]
pub(crate) fn get_x_lparam(lparam: LPARAM) -> i32 {
    (lparam.0 & 0xFFFF) as i32
}

#[cfg(target_os = "windows")]
pub(crate) fn get_y_lparam(lparam: LPARAM) -> i32 {
    ((lparam.0 >> 16) & 0xFFFF) as i32
}

#[cfg(target_os = "windows")]
#[command(rename_all = "snake_case")]
pub(crate) async fn get_win32_mouse_position<R: Runtime>(
    _window: Window<R>,
) -> Result<MousePosition, tauri::Error> {
    let mut point = POINT { x: 0, y: 0 };
    unsafe {
        GetCursorPos(&mut point).map_err(|e| anyhow!("{}", e))?;
    }
    Ok(MousePosition {
        x: point.x as f64,
        y: point.y as f64,
    })
}

#[cfg(target_os = "windows")]
#[command(rename_all = "snake_case")]
pub(crate) async fn get_lparam_mouse_position<R: Runtime>(
    _window: Window<R>,
) -> Result<MousePosition, tauri::Error> {
    let mut point = POINT { x: 0, y: 0 };
    unsafe {
        GetCursorPos(&mut point).map_err(|e| anyhow!("{}", e))?;
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
#[command(rename_all = "snake_case")]
pub(crate) async fn get_mapped_mouse_position<R: Runtime>(
    window: Window<R>,
) -> Result<MousePosition, tauri::Error> {
    let mut point = POINT { x: 0, y: 0 };
    unsafe {
        GetCursorPos(&mut point).map_err(|e| anyhow!("{}", e))?;
        
        let raw_handle = window.hwnd().map_err(|e| anyhow!("{}", e))?;
        let hwnd = HWND(raw_handle.0 as isize);
        
        if ScreenToClient(hwnd, &mut point).0 == 0 {
            return Err(anyhow!("Failed to map coordinates").into());
        }
    }
    
    Ok(MousePosition {
        x: point.x as f64,
        y: point.y as f64,
    })
}

#[cfg(target_os = "windows")]
#[command(rename_all = "snake_case")]
pub(crate) async fn map_window_points<R: Runtime>(
    window_from: Option<String>,
    window_to: Option<String>,
    app: AppHandle<R>,
    points: Vec<MousePosition>,
) -> Result<Vec<MousePosition>, tauri::Error> {
    let mut win_points: Vec<POINT> = points.iter()
        .map(|p| POINT { 
            x: p.x as i32, 
            y: p.y as i32 
        })
        .collect();

    unsafe {
        let hwnd_from = if let Some(label) = window_from {
            let window = app.get_webview_window(&label).ok_or_else(|| anyhow!("Window not found"))?;
            HWND(window.hwnd().map_err(|e| anyhow!("{}", e))?.0 as isize)
        } else {
            HWND_DESKTOP
        };

        let hwnd_to = if let Some(label) = window_to {
            let window = app.get_webview_window(&label).ok_or_else(|| anyhow!("Window not found"))?;
            HWND(window.hwnd().map_err(|e| anyhow!("{}", e))?.0 as isize)
        } else {
            HWND_DESKTOP
        };

        let result = MapWindowPoints(
            hwnd_from,
            hwnd_to,
            &mut win_points,
        );
        
        if result == 0 {
            return Err(anyhow!("Failed to map window points").into());
        }
    }

    Ok(win_points.into_iter()
        .map(|p| MousePosition {
            x: p.x as f64,
            y: p.y as f64,
        })
        .collect())
}

#[cfg(target_os = "windows")]
#[command(rename_all = "snake_case")]
pub(crate) async fn set_maximize_button_rect<R: Runtime>(
    _window: Window<R>,
    rect: ButtonRect,
) -> Result<(), tauri::Error> {
    *MAXIMIZE_BUTTON_RECT.lock().unwrap() = Some(rect);
    Ok(())
}

#[cfg(target_os = "windows")]
#[command(rename_all = "snake_case")]
pub(crate) async fn is_over_maximize_button<R: Runtime>(
    window: Window<R>,
) -> Result<bool, tauri::Error> {
    let mut point = POINT { x: 0, y: 0 };
    unsafe {
        GetCursorPos(&mut point).map_err(|e| anyhow!("{}", e))?;
        let raw_handle = window.hwnd().map_err(|e| anyhow!("{}", e))?;
        let hwnd = HWND(raw_handle.0 as isize);
        if ScreenToClient(hwnd, &mut point).0 == 0 {
            return Err(anyhow!("Failed to map coordinates").into());
        }
    }

    if let Some(rect) = &*MAXIMIZE_BUTTON_RECT.lock().unwrap() {
        Ok(point.x >= rect.left 
           && point.x <= rect.right 
           && point.y >= rect.top 
           && point.y <= rect.bottom)
    } else {
        Ok(false)
    }
}
