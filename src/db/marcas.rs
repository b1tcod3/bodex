use crate::models::{Marca, MarcaNueva};
use rusqlite::{params, Connection, Result};

/// Crea la tabla de marcas si no existe
pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS marcas (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nombre TEXT NOT NULL UNIQUE,
            descripcion TEXT,
            logo TEXT,
            rif TEXT UNIQUE
        )",
        [],
    )?;
    Ok(())
}

/// Obtener todas las marcas ordenadas por nombre
pub fn obtener_marcas(conn: &Connection) -> Result<Vec<Marca>> {
    let mut stmt =
        conn.prepare("SELECT id, nombre, descripcion, logo, rif FROM marcas ORDER BY nombre ASC")?;

    let marcas_iter = stmt.query_map([], |row| {
        Ok(Marca {
            id: row.get(0)?,
            nombre: row.get(1)?,
            descripcion: row.get(2)?,
            logo: row.get(3)?,
            rif: row.get(4)?,
        })
    })?;

    let mut resultado = Vec::new();
    for marca in marcas_iter {
        resultado.push(marca?);
    }
    Ok(resultado)
}

/// Crear una nueva marca
pub fn crear_marca(conn: &Connection, m: &MarcaNueva) -> Result<i64> {
    conn.execute(
        "INSERT INTO marcas (nombre, descripcion, logo, rif) VALUES (?1, ?2, ?3, ?4)",
        params![m.nombre, m.descripcion, m.logo, m.rif],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Obtener una marca específica por su ID
pub fn obtener_marca_por_id(conn: &Connection, id: i64) -> Result<Option<Marca>> {
    let mut stmt =
        conn.prepare("SELECT id, nombre, descripcion, logo, rif FROM marcas WHERE id = ?1")?;
    let mut rows = stmt.query(params![id])?;

    if let Some(row) = rows.next()? {
        Ok(Some(Marca {
            id: row.get(0)?,
            nombre: row.get(1)?,
            descripcion: row.get(2)?,
            logo: row.get(3)?,
            rif: row.get(4)?,
        }))
    } else {
        Ok(None)
    }
}

/// Actualizar una marca existente
pub fn actualizar_marca(conn: &Connection, m: &Marca) -> Result<bool> {
    let filas = conn.execute(
        "UPDATE marcas SET nombre = ?1, descripcion = ?2, logo = ?3, rif = ?4 WHERE id = ?5",
        params![m.nombre, m.descripcion, m.logo, m.rif, m.id],
    )?;
    Ok(filas > 0)
}

/// Eliminar una marca (Tener en cuenta que si hay productos asociados, el marca_id pasará a NULL)
pub fn eliminar_marca(conn: &Connection, id: i64) -> Result<bool> {
    let filas = conn.execute("DELETE FROM marcas WHERE id = ?1", params![id])?;
    Ok(filas > 0)
}

/// Inserta marcas iniciales si no existen (Seeder)
pub fn seed_marcas(conn: &Connection) -> Result<()> {
    // Verificar si ya existen marcas
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM marcas", [], |row| row.get(0))?;
    
    if count == 0 {
        // Insertar marca genérica por defecto
        conn.execute(
            "INSERT INTO marcas (nombre, descripcion) VALUES ('Genérico', 'Marca por defecto para productos sin marca específica')",
            [],
        )?;
        println!("Marca 'Genérico' insertada correctamente.");
    }
    Ok(())
}
