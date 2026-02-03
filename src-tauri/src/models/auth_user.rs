use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AuthUser {
    pub id: i32,
    pub nombre: String,
    pub rol: String,
    pub fecha_creacion: String,
    pub ultimo_login: Option<String>,
}
