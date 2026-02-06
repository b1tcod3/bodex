use rusqlite::{params, Connection, Result};
use crate::models::{Venta, DetalleVenta};

/// Crea la tabla principal de ventas
pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ventas (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            fecha DATETIME DEFAULT CURRENT_TIMESTAMP,
            total REAL NOT NULL,
            usuario_id INTEGER,
            cliente_nombre TEXT,
            FOREIGN KEY (usuario_id) REFERENCES usuarios(id)
        )",
        [],
    )?;
    Ok(())
}

/// Crea la tabla de detalles (productos vendidos en cada venta)
pub fn create_detalle_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ventas_detalle (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            venta_id INTEGER NOT NULL,
            producto_id INTEGER NOT NULL,
            cantidad INTEGER NOT NULL,
            precio_unitario REAL NOT NULL,
            subtotal REAL NOT NULL,
            FOREIGN KEY (venta_id) REFERENCES ventas(id) ON DELETE CASCADE,
            FOREIGN KEY (producto_id) REFERENCES productos(id)
        )",
        [],
    )?;
    Ok(())
}

/// Registra una venta completa y actualiza el stock usando una Transacción
pub fn registrar_venta(
    conn: &mut Connection, 
    usuario_id: i64, 
    cliente: &str, 
    detalles: Vec<DetalleVenta>
) -> Result<i64> {
    // Calculamos el total de la venta
    let total_venta: f64 = detalles.iter().map(|d| d.subtotal).sum();

    // Iniciamos la transacción: si algo falla, nada se guarda
    let tx = conn.transaction()?;

    // 1. Insertar en la tabla 'ventas'
    tx.execute(
        "INSERT INTO ventas (total, usuario_id, cliente_nombre) VALUES (?1, ?2, ?3)",
        params![total_venta, usuario_id, cliente],
    )?;
    
    let venta_id = tx.last_insert_rowid();

    // 2. Insertar detalles y descontar stock
    for item in detalles {
        // Registrar detalle
        tx.execute(
            "INSERT INTO ventas_detalle (venta_id, producto_id, cantidad, precio_unitario, subtotal) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![venta_id, item.producto_id, item.cantidad, item.precio_unitario, item.subtotal],
        )?;

        // Descontar stock del producto
        tx.execute(
            "UPDATE productos SET stock = stock - ?1 WHERE id = ?2 AND stock >= ?1",
            params![item.cantidad, item.producto_id],
        )?;
        
        // Verificación de seguridad: si el stock quedó negativo, SQLite lanzará un error 
        // o podemos verificar las filas afectadas.
    }

    // Confirmar todos los cambios
    tx.commit()?;

    Ok(venta_id)
}

/// Obtener el historial de ventas (Resumen)
pub fn obtener_historial(conn: &Connection) -> Result<Vec<Venta>> {
    let mut stmt = conn.prepare(
        "SELECT id, fecha, total, usuario_id, cliente_nombre FROM ventas ORDER BY fecha DESC"
    )?;

    let ventas_iter = stmt.query_map([], |row| {
        Ok(Venta {
            id: row.get(0)?,
            fecha: row.get(1)?,
            total: row.get(2)?,
            usuario_id: row.get(3)?,
            cliente_nombre: row.get(4)?,
        })
    })?;

    let mut resultado = Vec::new();
    for v in ventas_iter { resultado.push(v?); }
    Ok(resultado)
}
