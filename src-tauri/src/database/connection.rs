use rusqlite::{Connection, Result};
use std::fs;

pub fn connect() -> Result<Connection> {
    fs::create_dir_all("data").ok();

    let conn = Connection::open("data/app.db")?;
    Ok(conn)
}
