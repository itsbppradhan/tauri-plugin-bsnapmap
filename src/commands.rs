use tauri::{AppHandle, command, Runtime, Window};
use crate::models::*;
use crate::Result;
use crate::BsnapmapExt;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.bsnapmap().ping(payload)
}

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
