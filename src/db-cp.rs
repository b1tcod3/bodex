use rusqlite::{params, Connection, Result};
use crate::models::{Usuario, Rol, Producto};

pub fn open_connection() -> Result<Connection> {
    Connection::open("bodex.db")
}

pub fn init_db(conn: &Connection) -> Result<()> {
    // Tabla de Usuarios
    conn.execute(
        "CREATE TABLE IF NOT EXISTS usuarios (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            rol TEXT NOT NULL,
            activo INTEGER NOT NULL DEFAULT 1,
            ultimo_login DATETIME
        )",
        [],
    )?;

    // Tabla de Productos (Simplificada para el ejemplo)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS productos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nombre TEXT NOT NULL,
            precio_venta REAL NOT NULL,
            stock INTEGER NOT NULL
        )",
        [],
    )?;

    // Crear un usuario admin por defecto si no existe
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM usuarios", [], |r| r.get(0))?;
    if count == 0 {
        conn.execute(
            "INSERT INTO usuarios (username, password_hash, rol) VALUES (?1, ?2, ?3)",
            params!["admin", "admin", "Admin"],
        )?;
    }
    Ok(())
}

pub fn validar_usuario(conn: &Connection, user: &str, pass: &str) -> Result<Option<Usuario>> {
    let mut stmt = conn.prepare(
        "SELECT id, username, password_hash, rol, activo, ultimo_login FROM usuarios WHERE username = ?1"
    )?;
    
    let mut rows = stmt.query(params![user])?;
    if let Some(row) = rows.next()? {
        let stored_pass: String = row.get(2)?;
        if stored_pass == pass {
            return Ok(Some(Usuario {
                id: row.get(0)?,
                username: row.get(1)?,
                password_hash: stored_pass,
                rol: Rol::from(row.get::<_, String>(3)?),
                activo: row.get::<_, i32>(4)? != 0,
                ultimo_login: row.get(5)?,
            }));
        }
    }
    Ok(None)
}

pub fn obtener_productos(conn: &Connection) -> Result<Vec<Producto>> {
    let mut stmt = conn.prepare("SELECT id, nombre, precio_venta, stock FROM productos")?;
    let rows = stmt.query_map([], |row| {
        Ok(Producto {
            id: row.get(0)?,
            nombre: row.get(1)?,
            precio_venta: row.get(2)?,
            stock: row.get(3)?,
        })
    })?;
    let mut items = Vec::new();
    for item in rows { items.push(item?); }
    Ok(items)
}
