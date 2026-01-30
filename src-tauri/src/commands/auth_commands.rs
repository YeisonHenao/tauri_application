use crate::controllers::auth_controller::AuthController;

#[tauri::command]
pub fn login(usuario: String, password: String) -> Result<bool, String> {
    AuthController::login(usuario, password)
}
