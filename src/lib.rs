use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Bsnapmap;
#[cfg(mobile)]
use mobile::Bsnapmap;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the bsnapmap APIs.
pub trait BsnapmapExt<R: Runtime> {
  fn bsnapmap(&self) -> &Bsnapmap<R>;
}

impl<R: Runtime, T: Manager<R>> crate::BsnapmapExt<R> for T {
  fn bsnapmap(&self) -> &Bsnapmap<R> {
    self.state::<Bsnapmap<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("bsnapmap")
    .invoke_handler(tauri::generate_handler![
      commands::ping,
      commands::get_mouse_position,
      commands::get_win32_mouse_position,
      commands::get_lparam_mouse_position,
      commands::get_mapped_mouse_position,
      commands::set_maximize_button_rect
    ])
    .setup(|app, api| {
      #[cfg(mobile)]
      let bsnapmap = mobile::init(app, api)?;
      #[cfg(desktop)]
      let bsnapmap = desktop::init(app, api)?;
      app.manage(bsnapmap);
      Ok(())
    })
    .build()
}
