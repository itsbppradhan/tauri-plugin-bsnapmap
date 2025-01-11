const COMMANDS: &[&str] = &["ping", "get_mouse_position"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}
