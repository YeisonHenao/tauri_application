pub mod services;
pub mod controllers;
pub mod commands;

use commands::router::register;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(register())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
