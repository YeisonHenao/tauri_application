use crate::services::autentication::AutenticationService;

pub struct AuthController;

impl AuthController {
    pub fn login(usuario: String, password: String) -> Result<bool, String> {
        AutenticationService::login(&usuario, &password)
    }
}
