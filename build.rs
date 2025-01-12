const COMMANDS: &[&str] = &["ping", "get_mouse_position", "get_win32_mouse_position", "get_lparam_mouse_position"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}
