use rusqlite::{Connection, Result};
use std::path::Path;

pub fn establish_connection() -> Result<Connection> {
    let db_path = Path::new("database.db");
    let conn = Connection::open(db_path)?;
    
    // Create tables if they don't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    Ok(conn)
}