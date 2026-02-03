use crate::controllers::auth_controller;
use crate::models::auth_user::AuthUser;

#[tauri::command]
pub fn login(usuario: String, password: String) -> Result<AuthUser, String> {
    auth_controller::login(&usuario, &password)
}
