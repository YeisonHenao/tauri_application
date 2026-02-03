use crate::database::connection::connect;
use crate::repositories::user_repository::UserRepository;
use crate::utils::password::hash_password;

pub fn create_admin_if_not_exists() {
    let conn = connect().expect("Error al conectar a la base de datos");

    let admin_exists = UserRepository::find_for_login(&conn, "admin")
        .expect("Error consultando usuario")
        .is_some();

    if admin_exists {
        return;
    }

    let hashed_password =
        hash_password("admin123").expect("Error al generar hash de la contraseña");

    UserRepository::create_full(&conn, "admin", &hashed_password, "ADMIN")
        .expect("Error creando usuario administrador");

    println!("Usuario administrador creado correctamente");
}
