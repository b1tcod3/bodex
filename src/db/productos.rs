use crate::models::{Producto, ProductoConMarca, ProductoNuevo};
use rusqlite::{params, Connection, Result, Row};

/// Crea la tabla de productos si no existe
pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS productos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nombre TEXT NOT NULL,
            precio_neto REAL NOT NULL DEFAULT 0,
            precio_venta REAL NOT NULL DEFAULT 0,
            stock INTEGER NOT NULL DEFAULT 0,
            descripcion TEXT,
            peso REAL,
            tamano TEXT,
            unidad_medida TEXT,
            presentacion TEXT,
            codigo TEXT UNIQUE,
            activo INTEGER NOT NULL DEFAULT 1,
            fecha_vencimiento DATE,
            marca_id INTEGER,
            FOREIGN KEY (marca_id) REFERENCES marcas(id) ON DELETE SET NULL
        )",
        [],
    )?;
    Ok(())
}

/// Obtener todos los productos básicos
pub fn obtener_productos(conn: &Connection) -> Result<Vec<Producto>> {
    let mut stmt = conn.prepare(
        "SELECT id, nombre, precio_neto, precio_venta, stock, descripcion,
                peso, tamano, unidad_medida, presentacion, codigo, activo,
                fecha_vencimiento, marca_id
         FROM productos ORDER BY nombre ASC",
    )?;

    let productos = stmt.query_map([], mapear_producto)?;

    let mut resultado = Vec::new();
    for p in productos {
        resultado.push(p?);
    }
    Ok(resultado)
}

/// Obtener productos con el nombre de su marca (para la tabla principal de la UI)
pub fn obtener_productos_con_marca(conn: &Connection) -> Result<Vec<ProductoConMarca>> {
    let mut stmt = conn.prepare(
        "SELECT p.id, p.nombre, p.precio_neto, p.precio_venta, p.stock, 
                p.descripcion, p.peso, p.tamano, p.unidad_medida, p.presentacion, 
                p.codigo, p.activo, p.fecha_vencimiento, p.marca_id,
                m.nombre as marca_nombre
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
            peso: row.get(6)?,
            tamano: row.get(7)?,
            unidad_medida: row.get(8)?,
            presentacion: row.get(9)?,
            codigo: row.get(10)?,
            activo: row.get::<_, i32>(11)? != 0,
            fecha_vencimiento: row.get(12)?,
            marca_id: row.get(13)?,
            marca_nombre: row.get(14)?,
        })
    })?;

    let mut resultado = Vec::new();
    for p in productos {
        resultado.push(p?);
    }
    Ok(resultado)
}

/// Insertar un nuevo producto
pub fn crear_producto(conn: &Connection, p: &ProductoNuevo) -> Result<i64> {
    conn.execute(
        "INSERT INTO productos (
            nombre, precio_neto, precio_venta, stock, descripcion, 
            peso, tamano, unidad_medida, presentacion, codigo, 
            activo, fecha_vencimiento, marca_id
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
        params![
            p.nombre,
            p.precio_neto,
            p.precio_venta,
            p.stock,
            p.descripcion,
            p.peso,
            p.tamano,
            p.unidad_medida,
            p.presentacion,
            p.codigo,
            if p.activo { 1 } else { 0 },
            p.fecha_vencimiento,
            p.marca_id
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Eliminar producto por ID
pub fn eliminar_producto(conn: &Connection, id: i64) -> Result<bool> {
    let filas = conn.execute("DELETE FROM productos WHERE id = ?1", params![id])?;
    Ok(filas > 0)
}

/// Función auxiliar privada para mapear filas a la estructura Producto
fn mapear_producto(row: &Row) -> Result<Producto> {
    Ok(Producto {
        id: row.get(0)?,
        nombre: row.get(1)?,
        precio_neto: row.get(2)?,
        precio_venta: row.get(3)?,
        stock: row.get(4)?,
        descripcion: row.get(5)?,
        peso: row.get(6)?,
        tamano: row.get(7)?,
        unidad_medida: row.get(8)?,
        presentacion: row.get(9)?,
        codigo: row.get(10)?,
        activo: row.get::<_, i32>(11)? != 0,
        fecha_vencimiento: row.get(12)?,
        marca_id: row.get(13)?,
    })
}
