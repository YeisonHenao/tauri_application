mod commands;
mod controllers;
mod database;
mod models;
mod repositories;
mod services;
mod utils;

use services::autentication::admin_seed::create_admin_if_not_exists;

pub fn run() {
    create_admin_if_not_exists();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::auth_commands::login])
        .run(tauri::generate_context!())
        .expect("error running tauri");
}
