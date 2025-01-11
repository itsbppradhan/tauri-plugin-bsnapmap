use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<Bsnapmap<R>> {
  Ok(Bsnapmap(app.clone()))
}

/// Access to the bsnapmap APIs.
pub struct Bsnapmap<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Bsnapmap<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }
}
