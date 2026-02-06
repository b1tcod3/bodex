use rusqlite::{params, Connection, Result};
use crate::models::{Usuario, Rol};
use chrono::NaiveDateTime;

/// Crea la tabla de usuarios si no existe
pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS usuarios (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            rol TEXT NOT NULL CHECK(rol IN ('Admin', 'Operador', 'Vendedor')),
            activo INTEGER NOT NULL DEFAULT 1,
            ultimo_login DATETIME
        )",
        [],
    )?;
    Ok(())
}

/// Inserta un usuario administrador inicial si la tabla está vacía
pub fn seed_admin(conn: &Connection) -> Result<()> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM usuarios",
        [],
        |r| r.get(0)
    )?;

    if count == 0 {
        conn.execute(
            "INSERT INTO usuarios (username, password_hash, rol, activo) 
             VALUES (?1, ?2, ?3, ?4)",
            params!["admin", "admin", "Admin", 1],
        )?;
        println!("Usuario administrador inicial creado: admin/admin");
    }
    Ok(())
}

/// Valida las credenciales y devuelve el Usuario si es exitoso
pub fn validar_usuario(conn: &Connection, user: &str, pass: &str) -> Result<Option<Usuario>> {
    let mut stmt = conn.prepare(
        "SELECT id, username, password_hash, rol, activo, ultimo_login 
         FROM usuarios 
         WHERE username = ?1 AND activo = 1"
    )?;

    let mut rows = stmt.query(params![user])?;

    if let Some(row) = rows.next()? {
        let stored_hash: String = row.get(2)?;

        // Verificación de contraseña (texto plano por ahora, como en tus pruebas)
        if stored_hash == pass {
            // Actualizamos la fecha del último login
            let _ = conn.execute(
                "UPDATE usuarios SET ultimo_login = CURRENT_TIMESTAMP WHERE id = ?1",
                params![row.get::<_, i64>(0)?],
            );

            return Ok(Some(Usuario {
                id: row.get(0)?,
                username: row.get(1)?,
                password_hash: stored_hash,
                rol: Rol::from(row.get::<_, String>(3)?),
                activo: row.get::<_, i32>(4)? != 0,
                ultimo_login: row.get(5)?,
            }));
        }
    }
    
    Ok(None)
}

/// Crea un nuevo usuario en el sistema
pub fn crear_usuario(conn: &Connection, username: &str, password_hash: &str, rol: Rol) -> Result<i64> {
    conn.execute(
        "INSERT INTO usuarios (username, password_hash, rol) VALUES (?1, ?2, ?3)",
        params![username, password_hash, rol.to_string()],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Elimina un usuario por ID
pub fn eliminar_usuario(conn: &Connection, id: i64) -> Result<bool> {
    let filas = conn.execute("DELETE FROM usuarios WHERE id = ?1", params![id])?;
    Ok(filas > 0)
}
