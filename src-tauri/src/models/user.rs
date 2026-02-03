#[derive(Debug)]
pub struct UserDb {
    pub id: i32,
    pub nombre: String,
    pub password: String,
    pub rol: String,
    pub fecha_creacion: String,
    pub ultimo_login: Option<String>,
}
