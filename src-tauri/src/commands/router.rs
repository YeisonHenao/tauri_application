use crate::commands::auth_commands;
use tauri::ipc::Invoke;

pub fn register<R: tauri::Runtime>()
    -> impl Fn(Invoke<R>) -> bool + Send + Sync + 'static
{
    tauri::generate_handler![
        auth_commands::login
    ]
}
