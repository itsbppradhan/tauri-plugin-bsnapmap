const COMMANDS: &[&str] = &[
    "ping", 
    "get_mouse_position", 
    "get_win32_mouse_position", 
    "get_lparam_mouse_position", 
    "get_mapped_mouse_position",
    "set_maximize_button_rect",
    "is_over_maximize_button"
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
