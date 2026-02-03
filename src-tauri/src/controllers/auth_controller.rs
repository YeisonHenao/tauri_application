use crate::database::connection::connect;
use crate::models::auth_user::AuthUser;
use crate::repositories::user_repository::UserRepository;
use crate::utils::password::verify_password;
use chrono::Utc;

pub fn login(nombre: &str, password: &str) -> Result<AuthUser, String> {
    let conn = connect().map_err(|_| "DB error")?;

    let data =
        UserRepository::find_for_login(&conn, nombre).map_err(|_| "Error consultando usuario")?;

    let (user, password_hash) = match data {
        Some(v) => v,
        None => return Err("Usuario no encontrado".into()),
    };

    let valid =
        verify_password(password, &password_hash).map_err(|_| "Error verificando contraseña")?;

    if !valid {
        return Err("Credenciales incorrectas".into());
    }

    // actualizar último login
    let now = Utc::now().to_rfc3339();
    UserRepository::update_last_login(&conn, user.id, &now)
        .map_err(|_| "Error actualizando último login")?;

    Ok(AuthUser {
        ultimo_login: Some(now),
        ..user
    })
}
