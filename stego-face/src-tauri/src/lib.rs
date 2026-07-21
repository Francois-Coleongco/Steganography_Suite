use cli_entry::{add_entry, read_entry_handler};

mod cli_entry;

#[tauri::command]
fn invoke_add_entry(master_password: String, data: String, file_path: String) -> Result<(), String> {
    add_entry(master_password, data, file_path)
}

#[tauri::command]
fn invoke_read_entry(master_password: String, file_path: String) -> Result<String, String> {
    read_entry_handler(master_password, &file_path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            invoke_add_entry,
            invoke_read_entry
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
