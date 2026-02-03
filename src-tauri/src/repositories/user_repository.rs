use crate::models::auth_user::AuthUser;
use crate::models::user::UserDb;
use rusqlite::{params, Connection, Result};

pub struct UserRepository;

impl UserRepository {
    pub fn create_full(conn: &Connection, nombre: &str, password: &str, rol: &str) -> Result<()> {
        conn.execute(
            "
                INSERT INTO users (
                    nombre,
                    password,
                    rol,
                    fecha_creacion,
                    ultimo_login
                )
                VALUES (
                    ?1,
                    ?2,
                    ?3,
                    datetime('now'),
                    datetime('now')
                )
                ",
            params![nombre, password, rol],
        )?;

        Ok(())
    }

    // CREATE (admin o usuario normal)
    pub fn create(
        conn: &Connection,
        nombre: &str,
        password: &str,
        rol: &str,
        fecha_creacion: &str,
    ) -> Result<()> {
        conn.execute(
            "INSERT INTO users (nombre, password, rol, fecha_creacion)
             VALUES (?, ?, ?, ?)",
            params![nombre, password, rol, fecha_creacion],
        )?;
        Ok(())
    }

    // READ completo (uso interno)
    pub fn find_by_nombre(conn: &Connection, nombre: &str) -> Result<Option<UserDb>> {
        let mut stmt = conn.prepare(
            "SELECT id, nombre, password, rol, fecha_creacion, ultimo_login
             FROM users WHERE nombre = ?",
        )?;

        let result = stmt.query_row([nombre], |row| {
            Ok(UserDb {
                id: row.get(0)?,
                nombre: row.get(1)?,
                password: row.get(2)?,
                rol: row.get(3)?,
                fecha_creacion: row.get(4)?,
                ultimo_login: row.get(5)?,
            })
        });

        match result {
            Ok(user) => Ok(Some(user)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    // LOGIN (lo que necesita autenticación)
    pub fn find_for_login(conn: &Connection, nombre: &str) -> Result<Option<(AuthUser, String)>> {
        let mut stmt = conn.prepare(
            "SELECT id, nombre, password, rol, fecha_creacion, ultimo_login
             FROM users WHERE nombre = ?",
        )?;

        let result = stmt.query_row([nombre], |row| {
            Ok((
                AuthUser {
                    id: row.get(0)?,
                    nombre: row.get(1)?,
                    rol: row.get(3)?,
                    fecha_creacion: row.get(4)?,
                    ultimo_login: row.get(5)?,
                },
                row.get(2)?, // password hash
            ))
        });

        match result {
            Ok(data) => Ok(Some(data)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    // UPDATE último login
    pub fn update_last_login(conn: &Connection, id: i32, timestamp: &str) -> Result<()> {
        conn.execute(
            "UPDATE users SET ultimo_login = ? WHERE id = ?",
            params![timestamp, id],
        )?;
        Ok(())
    }
}
