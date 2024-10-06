use std::env;

use cli_entry::add_entry;

mod cli_entry;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn invoke_add_entry(
    mut master_password: String,
    mut data_name: String,
    mut data: String,
    mut file_path: String,
) {
    add_entry(master_password, data_name, data, file_path);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![invoke_add_entry])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
