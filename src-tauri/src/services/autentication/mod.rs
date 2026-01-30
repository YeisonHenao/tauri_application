pub struct AutenticationService;

impl AutenticationService {
    pub fn login(usuario: &str, password: &str) -> Result<bool, String> {
        if usuario.is_empty() || password.is_empty() {
            return Err("Credenciales inválidas".into());
        }

        Ok(usuario == "admin" && password == "1234")
    }
}
