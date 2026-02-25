use crate::models::{Producto, ProductoConMarca, ProductoNuevo};
use rusqlite::{params, Connection, Result, Row};

/// Crea la tabla de productos actualizada a la nueva arquitectura
pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS productos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nombre TEXT NOT NULL,
            precio_neto REAL NOT NULL DEFAULT 0,
            precio_venta REAL NOT NULL DEFAULT 0,
            stock INTEGER NOT NULL DEFAULT 0,
            descripcion TEXT,
            codigo TEXT UNIQUE,
            activo INTEGER NOT NULL DEFAULT 1,
            marca_id INTEGER,
            -- Nuevos campos de Enums y Cantidades
            medida_p_id INTEGER NOT NULL,
            cantidad_p REAL NOT NULL DEFAULT 0,
            medida_s_id INTEGER,
            cantidad_s REAL,
            empaque_id INTEGER NOT NULL,
            -- Categoría y subcategoría
            categoria_id INTEGER NOT NULL DEFAULT 1,
            subcategoria_id INTEGER NOT NULL DEFAULT 1,
            FOREIGN KEY (marca_id) REFERENCES marcas(id) ON DELETE SET NULL
        )",
        [],
    )?;
    Ok(())
}

/// Obtener todos los productos básicos (mapeo directo a struct Producto)
pub fn obtener_productos(conn: &Connection) -> Result<Vec<Producto>> {
    let mut stmt = conn.prepare(
        "SELECT id, nombre, precio_neto, precio_venta, stock, descripcion,
                codigo, activo, marca_id, medida_p_id, cantidad_p, 
                medida_s_id, cantidad_s, empaque_id, categoria_id, subcategoria_id
         FROM productos ORDER BY nombre ASC",
    )?;

    let productos = stmt.query_map([], mapear_producto)?;

    let mut resultado = Vec::new();
    for p in productos {
        resultado.push(p?);
    }
    Ok(resultado)
}

/// Obtener productos con marca para la UI
pub fn obtener_productos_con_marca(conn: &Connection) -> Result<Vec<ProductoConMarca>> {
    let mut stmt = conn.prepare(
        "SELECT p.id, p.nombre, p.precio_neto, p.precio_venta, p.stock, 
                p.descripcion, p.codigo, p.activo, p.marca_id, m.nombre as marca_nombre,
                p.medida_p_id, p.cantidad_p, p.empaque_id, p.categoria_id, p.subcategoria_id
         FROM productos p
         LEFT JOIN marcas m ON p.marca_id = m.id
         ORDER BY p.nombre ASC",
    )?;

    let productos = stmt.query_map([], |row| {
        Ok(ProductoConMarca {
            id: row.get(0)?,
            nombre: row.get(1)?,
            precio_neto: row.get(2)?,
            precio_venta: row.get(3)?,
            stock: row.get(4)?,
            descripcion: row.get(5)?,
            codigo: row.get(6)?,
            activo: row.get::<_, i32>(7)? != 0,
            marca_id: row.get(8)?,
            marca_nombre: row.get(9)?,
            medida_p_id: row.get(10)?,
            cantidad_p: row.get(11)?,
            empaque_id: row.get(12)?,
            categoria_id: row.get(13)?,
            subcategoria_id: row.get(14)?,
        })
    })?;

    let mut resultado = Vec::new();
    for p in productos {
        resultado.push(p?);
    }
    Ok(resultado)
}

/// Insertar un nuevo producto usando la estructura ProductoNuevo
pub fn crear_producto(conn: &Connection, p: &ProductoNuevo) -> Result<i64> {
    conn.execute(
        "INSERT INTO productos (
            nombre, precio_neto, precio_venta, stock, descripcion, 
            codigo, activo, marca_id, medida_p_id, cantidad_p, 
            medida_s_id, cantidad_s, empaque_id, categoria_id, subcategoria_id
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
        params![
            p.nombre,
            p.precio_neto,
            p.precio_venta,
            p.stock,
            p.descripcion,
            p.codigo,
            if p.activo { 1 } else { 0 },
            p.marca_id,
            p.medida_p_id,
            p.cantidad_p,
            p.medida_s_id,
            p.cantidad_s,
            p.empaque_id,
            p.categoria_id,
            p.subcategoria_id
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Eliminar producto por ID
pub fn eliminar_producto(conn: &Connection, id: i64) -> Result<bool> {
    let filas = conn.execute("DELETE FROM productos WHERE id = ?1", params![id])?;
    Ok(filas > 0)
}

/// Verificar si existe un producto con el código/SKU dado
/// Retorna true si el SKU ya existe en la base de datos
pub fn existe_sku(conn: &Connection, codigo: &str) -> Result<bool> {
    // Ignorar códigos vacíos
    if codigo.trim().is_empty() {
        return Ok(false);
    }
    
    let mut stmt = conn.prepare(
        "SELECT COUNT(*) FROM productos WHERE codigo = ?1"
    )?;
    
    let count: i32 = stmt.query_row(params![codigo.trim()], |row| row.get(0))?;
    Ok(count > 0)
}

/// Mapeo limpio de filas SQL a la estructura Producto
fn mapear_producto(row: &Row) -> Result<Producto> {
    Ok(Producto {
        id: row.get(0)?,
        nombre: row.get(1)?,
        precio_neto: row.get(2)?,
        precio_venta: row.get(3)?,
        stock: row.get(4)?,
        descripcion: row.get(5)?,
        codigo: row.get(6)?,
        activo: row.get::<_, i32>(7)? != 0,
        marca_id: row.get(8)?,
        medida_p_id: row.get(9)?,
        cantidad_p: row.get(10)?,
        medida_s_id: row.get(11)?,
        cantidad_s: row.get(12)?,
        empaque_id: row.get(13)?,
        categoria_id: row.get(14)?,
        subcategoria_id: row.get(15)?,
    })
}
