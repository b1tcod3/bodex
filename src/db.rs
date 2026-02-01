use rusqlite::{params, Connection, Result, Row};
use chrono::{NaiveDate, NaiveDateTime};

// ==================== ESTRUCTURAS DE DATOS ====================

#[derive(Debug, Clone)]
pub struct Marca {
    pub id: i64,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub logo: Option<String>,
    pub rif: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MarcaNueva {
    pub nombre: String,
    pub descripcion: Option<String>,
    pub logo: Option<String>,
    pub rif: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Producto {
    pub id: i64,
    pub nombre: String,
    pub precio_neto: f64,
    pub precio_venta: f64,
    pub stock: i64,
    pub descripcion: Option<String>,
    pub peso: Option<f64>,
    pub tamano: Option<String>,
    pub unidad_medida: Option<String>,
    pub presentacion: Option<String>,
    pub codigo: Option<String>,
    pub activo: bool,
    pub fecha_vencimiento: Option<NaiveDate>,
    pub marca_id: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct ProductoNuevo {
    pub nombre: String,
    pub precio_neto: f64,
    pub precio_venta: f64,
    pub stock: i64,
    pub descripcion: Option<String>,
    pub peso: Option<f64>,
    pub tamano: Option<String>,
    pub unidad_medida: Option<String>,
    pub presentacion: Option<String>,
    pub codigo: Option<String>,
    pub activo: bool,
    pub fecha_vencimiento: Option<NaiveDate>,
    pub marca_id: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct ProductoConMarca {
    pub id: i64,
    pub nombre: String,
    pub precio_neto: f64,
    pub precio_venta: f64,
    pub stock: i64,
    pub descripcion: Option<String>,
    pub peso: Option<f64>,
    pub tamano: Option<String>,
    pub unidad_medida: Option<String>,
    pub presentacion: Option<String>,
    pub codigo: Option<String>,
    pub activo: bool,
    pub fecha_vencimiento: Option<NaiveDate>,
    pub marca_id: Option<i64>,
    pub marca_nombre: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Venta {
    pub id: i64,
    pub fecha: NaiveDateTime,
    pub total: f64,
}

#[derive(Debug, Clone)]
pub struct DetalleVenta {
    pub id: i64,
    pub venta_id: i64,
    pub producto_id: i64,
    pub cantidad: i64,
    pub precio_unitario: f64,
    pub subtotal: f64,
}

#[derive(Debug, Clone)]
pub struct DetalleVentaConProducto {
    pub id: i64,
    pub venta_id: i64,
    pub producto_id: i64,
    pub nombre_producto: String,
    pub cantidad: i64,
    pub precio_unitario: f64,
    pub subtotal: f64,
}

#[derive(Debug, Clone)]
pub struct VentaConDetalles {
    pub venta: Venta,
    pub detalles: Vec<DetalleVentaConProducto>,
}

// ==================== INICIALIZACIÓN DE BASE DE DATOS ====================

pub fn init_db(conn: &Connection) -> Result<()> {
    // Tabla de marcas (debe crearse antes de productos por la foreign key)
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

    // Tabla de productos
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

    // Tabla de ventas
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ventas (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            fecha DATETIME DEFAULT CURRENT_TIMESTAMP,
            total REAL NOT NULL DEFAULT 0
        )",
        [],
    )?;

    // Tabla de detalles de venta
    conn.execute(
        "CREATE TABLE IF NOT EXISTS detalle_ventas (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            venta_id INTEGER NOT NULL,
            producto_id INTEGER NOT NULL,
            cantidad INTEGER NOT NULL,
            precio_unitario REAL NOT NULL,
            subtotal REAL NOT NULL,
            FOREIGN KEY (venta_id) REFERENCES ventas(id) ON DELETE CASCADE,
            FOREIGN KEY (producto_id) REFERENCES productos(id) ON DELETE RESTRICT
        )",
        [],
    )?;

    Ok(())
}

pub fn open_connection() -> Result<Connection> {
    Connection::open("bodex.db")
}

// ==================== FUNCIONES CRUD PARA MARCAS ====================

/// Crear una nueva marca
pub fn crear_marca(conn: &Connection, marca: &MarcaNueva) -> Result<i64> {
    conn.execute(
        "INSERT INTO marcas (nombre, descripcion, logo, rif) VALUES (?1, ?2, ?3, ?4)",
        params![marca.nombre, marca.descripcion, marca.logo, marca.rif],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Obtener todas las marcas
pub fn obtener_marcas(conn: &Connection) -> Result<Vec<Marca>> {
    let mut stmt = conn.prepare(
        "SELECT id, nombre, descripcion, logo, rif FROM marcas ORDER BY nombre"
    )?;
    
    let marcas = stmt.query_map([], |row| {
        Ok(Marca {
            id: row.get(0)?,
            nombre: row.get(1)?,
            descripcion: row.get(2)?,
            logo: row.get(3)?,
            rif: row.get(4)?,
        })
    })?;
    
    marcas.collect()
}

/// Obtener una marca por ID
pub fn obtener_marca_por_id(conn: &Connection, id: i64) -> Result<Option<Marca>> {
    let mut stmt = conn.prepare(
        "SELECT id, nombre, descripcion, logo, rif FROM marcas WHERE id = ?1"
    )?;
    
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

/// Buscar marcas por nombre
pub fn buscar_marcas_por_nombre(conn: &Connection, nombre: &str) -> Result<Vec<Marca>> {
    let mut stmt = conn.prepare(
        "SELECT id, nombre, descripcion, logo, rif FROM marcas 
         WHERE nombre LIKE ?1 ORDER BY nombre"
    )?;
    
    let marcas = stmt.query_map(params![format!("%{}%", nombre)], |row| {
        Ok(Marca {
            id: row.get(0)?,
            nombre: row.get(1)?,
            descripcion: row.get(2)?,
            logo: row.get(3)?,
            rif: row.get(4)?,
        })
    })?;
    
    marcas.collect()
}

/// Actualizar una marca existente
pub fn actualizar_marca(conn: &Connection, marca: &Marca) -> Result<bool> {
    let filas_afectadas = conn.execute(
        "UPDATE marcas SET nombre = ?1, descripcion = ?2, logo = ?3, rif = ?4 WHERE id = ?5",
        params![marca.nombre, marca.descripcion, marca.logo, marca.rif, marca.id],
    )?;
    
    Ok(filas_afectadas > 0)
}

/// Eliminar una marca por ID
pub fn eliminar_marca(conn: &Connection, id: i64) -> Result<bool> {
    let filas_afectadas = conn.execute(
        "DELETE FROM marcas WHERE id = ?1",
        params![id],
    )?;
    
    Ok(filas_afectadas > 0)
}

// ==================== FUNCIONES CRUD PARA PRODUCTOS ====================

/// Crear un nuevo producto
pub fn crear_producto(conn: &Connection, producto: &ProductoNuevo) -> Result<i64> {
    conn.execute(
        "INSERT INTO productos (
            nombre, precio_neto, precio_venta, stock, descripcion, 
            peso, tamano, unidad_medida, presentacion, codigo, 
            activo, fecha_vencimiento, marca_id
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
        params![
            producto.nombre, 
            producto.precio_neto, 
            producto.precio_venta, 
            producto.stock, 
            producto.descripcion,
            producto.peso, 
            producto.tamano, 
            producto.unidad_medida, 
            producto.presentacion,
            producto.codigo, 
            producto.activo as i32, 
            producto.fecha_vencimiento, 
            producto.marca_id
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Obtener todos los productos
pub fn obtener_productos(conn: &Connection) -> Result<Vec<Producto>> {
    let mut stmt = conn.prepare(
        "SELECT 
            id, nombre, precio_neto, precio_venta, stock, descripcion,
            peso, tamano, unidad_medida, presentacion, codigo, activo,
            fecha_vencimiento, marca_id
         FROM productos 
         ORDER BY nombre"
    )?;
    
    let productos = stmt.query_map([], mapear_producto)?;
    productos.collect()
}

/// Obtener todos los productos con información de su marca
pub fn obtener_productos_con_marca(conn: &Connection) -> Result<Vec<ProductoConMarca>> {
    let mut stmt = conn.prepare(
        "SELECT 
            p.id, p.nombre, p.precio_neto, p.precio_venta, p.stock, 
            p.descripcion, p.peso, p.tamano, p.unidad_medida, p.presentacion, 
            p.codigo, p.activo, p.fecha_vencimiento, p.marca_id,
            m.nombre as marca_nombre
         FROM productos p
         LEFT JOIN marcas m ON p.marca_id = m.id
         ORDER BY p.nombre"
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
    
    productos.collect()
}

/// Obtener un producto por ID
pub fn obtener_producto_por_id(conn: &Connection, id: i64) -> Result<Option<Producto>> {
    let mut stmt = conn.prepare(
        "SELECT 
            id, nombre, precio_neto, precio_venta, stock, descripcion,
            peso, tamano, unidad_medida, presentacion, codigo, activo,
            fecha_vencimiento, marca_id
         FROM productos WHERE id = ?1"
    )?;
    
    let mut rows = stmt.query(params![id])?;
    
    if let Some(row) = rows.next()? {
        Ok(Some(mapear_producto(row)?))
    } else {
        Ok(None)
    }
}

/// Obtener un producto por código
pub fn obtener_producto_por_codigo(conn: &Connection, codigo: &str) -> Result<Option<Producto>> {
    let mut stmt = conn.prepare(
        "SELECT 
            id, nombre, precio_neto, precio_venta, stock, descripcion,
            peso, tamano, unidad_medida, presentacion, codigo, activo,
            fecha_vencimiento, marca_id
         FROM productos WHERE codigo = ?1"
    )?;
    
    let mut rows = stmt.query(params![codigo])?;
    
    if let Some(row) = rows.next()? {
        Ok(Some(mapear_producto(row)?))
    } else {
        Ok(None)
    }
}

/// Buscar productos por nombre
pub fn buscar_productos_por_nombre(conn: &Connection, nombre: &str) -> Result<Vec<Producto>> {
    let mut stmt = conn.prepare(
        "SELECT 
            id, nombre, precio_neto, precio_venta, stock, descripcion,
            peso, tamano, unidad_medida, presentacion, codigo, activo,
            fecha_vencimiento, marca_id
         FROM productos 
         WHERE nombre LIKE ?1 ORDER BY nombre"
    )?;
    
    let productos = stmt.query_map(params![format!("%{}%", nombre)], mapear_producto)?;
    productos.collect()
}

/// Buscar productos por marca
pub fn buscar_productos_por_marca(conn: &Connection, marca_id: i64) -> Result<Vec<Producto>> {
    let mut stmt = conn.prepare(
        "SELECT 
            id, nombre, precio_neto, precio_venta, stock, descripcion,
            peso, tamano, unidad_medida, presentacion, codigo, activo,
            fecha_vencimiento, marca_id
         FROM productos 
         WHERE marca_id = ?1 ORDER BY nombre"
    )?;
    
    let productos = stmt.query_map(params![marca_id], mapear_producto)?;
    productos.collect()
}

/// Actualizar un producto existente
pub fn actualizar_producto(conn: &Connection, producto: &Producto) -> Result<bool> {
    let filas_afectadas = conn.execute(
        "UPDATE productos SET 
            nombre = ?1, precio_neto = ?2, precio_venta = ?3, stock = ?4, 
            descripcion = ?5, peso = ?6, tamano = ?7, unidad_medida = ?8, 
            presentacion = ?9, codigo = ?10, activo = ?11, 
            fecha_vencimiento = ?12, marca_id = ?13
         WHERE id = ?14",
        params![
            producto.nombre, 
            producto.precio_neto, 
            producto.precio_venta, 
            producto.stock,
            producto.descripcion,
            producto.peso, 
            producto.tamano, 
            producto.unidad_medida, 
            producto.presentacion,
            producto.codigo, 
            producto.activo as i32, 
            producto.fecha_vencimiento, 
            producto.marca_id,
            producto.id
        ],
    )?;
    
    Ok(filas_afectadas > 0)
}

/// Eliminar un producto por ID
pub fn eliminar_producto(conn: &Connection, id: i64) -> Result<bool> {
    let filas_afectadas = conn.execute(
        "DELETE FROM productos WHERE id = ?1",
        params![id],
    )?;
    
    Ok(filas_afectadas > 0)
}

/// Actualizar solo el stock de un producto
pub fn actualizar_stock(conn: &Connection, producto_id: i64, nuevo_stock: i64) -> Result<bool> {
    let filas_afectadas = conn.execute(
        "UPDATE productos SET stock = ?1 WHERE id = ?2",
        params![nuevo_stock, producto_id],
    )?;
    
    Ok(filas_afectadas > 0)
}

/// Reducir stock de un producto (para ventas)
pub fn reducir_stock(conn: &Connection, producto_id: i64, cantidad: i64) -> Result<bool> {
    let filas_afectadas = conn.execute(
        "UPDATE productos SET stock = stock - ?1 WHERE id = ?2 AND stock >= ?1",
        params![cantidad, producto_id],
    )?;
    
    Ok(filas_afectadas > 0)
}

/// Incrementar stock de un producto (para devoluciones o compras)
pub fn incrementar_stock(conn: &Connection, producto_id: i64, cantidad: i64) -> Result<bool> {
    let filas_afectadas = conn.execute(
        "UPDATE productos SET stock = stock + ?1 WHERE id = ?2",
        params![cantidad, producto_id],
    )?;
    
    Ok(filas_afectadas > 0)
}

/// Activar o desactivar un producto
pub fn activar_producto(conn: &Connection, producto_id: i64, activo: bool) -> Result<bool> {
    let filas_afectadas = conn.execute(
        "UPDATE productos SET activo = ?1 WHERE id = ?2",
        params![activo as i32, producto_id],
    )?;
    
    Ok(filas_afectadas > 0)
}

/// Función auxiliar para mapear una fila a un Producto
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

// ==================== FUNCIONES PARA VENTAS ====================

/// Estructura para crear un item de venta
#[derive(Debug, Clone)]
pub struct ItemVenta {
    pub producto_id: i64,
    pub cantidad: i64,
    pub precio_unitario: f64,
}

/// Registrar una nueva venta con sus detalles
/// Esta función ejecuta todo en una transacción para garantizar la integridad
pub fn registrar_venta(conn: &mut Connection, items: &[ItemVenta]) -> Result<i64> {
    let tx = conn.transaction()?;
    
    // Calcular el total de la venta
    let total: f64 = items.iter()
        .map(|item| item.precio_unitario * item.cantidad as f64)
        .sum();
    
    // Insertar la venta
    tx.execute(
        "INSERT INTO ventas (total) VALUES (?1)",
        params![total],
    )?;
    
    let venta_id = tx.last_insert_rowid();
    
    // Insertar los detalles y actualizar stock
    for item in items {
        let subtotal = item.precio_unitario * item.cantidad as f64;
        
        // Insertar detalle de venta
        tx.execute(
            "INSERT INTO detalle_ventas (venta_id, producto_id, cantidad, precio_unitario, subtotal) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![venta_id, item.producto_id, item.cantidad, item.precio_unitario, subtotal],
        )?;
        
        // Reducir el stock del producto
        let filas_afectadas = tx.execute(
            "UPDATE productos SET stock = stock - ?1 WHERE id = ?2 AND stock >= ?1",
            params![item.cantidad, item.producto_id],
        )?;
        
        if filas_afectadas == 0 {
            return Err(rusqlite::Error::ExecuteReturnedResults);
        }
    }
    
    tx.commit()?;
    Ok(venta_id)
}

/// Obtener todas las ventas
pub fn obtener_ventas(conn: &Connection) -> Result<Vec<Venta>> {
    let mut stmt = conn.prepare(
        "SELECT id, fecha, total FROM ventas ORDER BY fecha DESC"
    )?;
    
    let ventas = stmt.query_map([], |row| {
        Ok(Venta {
            id: row.get(0)?,
            fecha: row.get(1)?,
            total: row.get(2)?,
        })
    })?;
    
    ventas.collect()
}

/// Obtener una venta por ID con todos sus detalles
pub fn obtener_venta_con_detalles(conn: &Connection, venta_id: i64) -> Result<Option<VentaConDetalles>> {
    // Obtener la venta
    let venta = {
        let mut stmt = conn.prepare(
            "SELECT id, fecha, total FROM ventas WHERE id = ?1"
        )?;
        
        let mut rows = stmt.query(params![venta_id])?;
        
        if let Some(row) = rows.next()? {
            Venta {
                id: row.get(0)?,
                fecha: row.get(1)?,
                total: row.get(2)?,
            }
        } else {
            return Ok(None);
        }
    };
    
    // Obtener los detalles con información del producto
    let mut stmt = conn.prepare(
        "SELECT 
            d.id, d.venta_id, d.producto_id, p.nombre,
            d.cantidad, d.precio_unitario, d.subtotal
         FROM detalle_ventas d
         JOIN productos p ON d.producto_id = p.id
         WHERE d.venta_id = ?1"
    )?;
    
    let detalles = stmt.query_map(params![venta_id], |row| {
        Ok(DetalleVentaConProducto {
            id: row.get(0)?,
            venta_id: row.get(1)?,
            producto_id: row.get(2)?,
            nombre_producto: row.get(3)?,
            cantidad: row.get(4)?,
            precio_unitario: row.get(5)?,
            subtotal: row.get(6)?,
        })
    })?;
    
    Ok(Some(VentaConDetalles {
        venta,
        detalles: detalles.collect::<Result<Vec<_>>>()?,
    }))
}

/// Obtener los detalles de una venta específica
pub fn obtener_detalles_venta(conn: &Connection, venta_id: i64) -> Result<Vec<DetalleVentaConProducto>> {
    let mut stmt = conn.prepare(
        "SELECT 
            d.id, d.venta_id, d.producto_id, p.nombre,
            d.cantidad, d.precio_unitario, d.subtotal
         FROM detalle_ventas d
         JOIN productos p ON d.producto_id = p.id
         WHERE d.venta_id = ?1"
    )?;
    
    let detalles = stmt.query_map(params![venta_id], |row| {
        Ok(DetalleVentaConProducto {
            id: row.get(0)?,
            venta_id: row.get(1)?,
            producto_id: row.get(2)?,
            nombre_producto: row.get(3)?,
            cantidad: row.get(4)?,
            precio_unitario: row.get(5)?,
            subtotal: row.get(6)?,
        })
    })?;
    
    detalles.collect()
}

/// Eliminar una venta (restaura el stock automáticamente por TRIGGER si se configura,
/// o manualmente aquí)
pub fn eliminar_venta(conn: &mut Connection, venta_id: i64) -> Result<bool> {
    let tx = conn.transaction()?;
    
    // Primero restaurar el stock de todos los productos
    let mut stmt = tx.prepare(
        "SELECT producto_id, cantidad FROM detalle_ventas WHERE venta_id = ?1"
    )?;
    
    let items: Vec<(i64, i64)> = stmt.query_map(params![venta_id], |row| {
        Ok((row.get(0)?, row.get(1)?))
    })?.collect::<Result<Vec<_>>>()?;
    
    drop(stmt);
    
    // Restaurar stock
    for (producto_id, cantidad) in items {
        tx.execute(
            "UPDATE productos SET stock = stock + ?1 WHERE id = ?2",
            params![cantidad, producto_id],
        )?;
    }
    
    // Eliminar la venta (los detalles se eliminan por CASCADE)
    let filas_afectadas = tx.execute(
        "DELETE FROM ventas WHERE id = ?1",
        params![venta_id],
    )?;
    
    tx.commit()?;
    
    Ok(filas_afectadas > 0)
}

/// Obtener estadísticas de ventas por período (útil para reportes)
pub fn obtener_estadisticas_ventas(
    conn: &Connection,
    fecha_desde: &str,
    fecha_hasta: &str
) -> Result<(i64, f64)> {
    let mut stmt = conn.prepare(
        "SELECT COUNT(*), COALESCE(SUM(total), 0) FROM ventas 
         WHERE date(fecha) BETWEEN ?1 AND ?2"
    )?;
    
    let mut rows = stmt.query(params![fecha_desde, fecha_hasta])?;
    
    if let Some(row) = rows.next()? {
        let cantidad: i64 = row.get(0)?;
        let total: f64 = row.get(1)?;
        Ok((cantidad, total))
    } else {
        Ok((0, 0.0))
    }
}

/// Obtener productos más vendidos
pub fn obtener_productos_mas_vendidos(conn: &Connection, limite: i64) -> Result<Vec<(String, i64, f64)>> {
    let mut stmt = conn.prepare(
        "SELECT p.nombre, SUM(d.cantidad) as cantidad_vendida, SUM(d.subtotal) as total_vendido
         FROM detalle_ventas d
         JOIN productos p ON d.producto_id = p.id
         GROUP BY d.producto_id
         ORDER BY cantidad_vendida DESC
         LIMIT ?1"
    )?;
    
    let resultados = stmt.query_map(params![limite], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, i64>(1)?,
            row.get::<_, f64>(2)?,
        ))
    })?;
    
    resultados.collect()
}

/// Verificar si hay stock suficiente para una venta
pub fn verificar_stock_suficiente(conn: &Connection, producto_id: i64, cantidad_requerida: i64) -> Result<bool> {
    let mut stmt = conn.prepare(
        "SELECT stock FROM productos WHERE id = ?1"
    )?;
    
    let mut rows = stmt.query(params![producto_id])?;
    
    if let Some(row) = rows.next()? {
        let stock: i64 = row.get(0)?;
        Ok(stock >= cantidad_requerida)
    } else {
        Ok(false)
    }
}

/// Obtener productos próximos a vencer
pub fn obtener_productos_por_vencer(conn: &Connection, dias: i64) -> Result<Vec<Producto>> {
    let mut stmt = conn.prepare(
        "SELECT 
            id, nombre, precio_neto, precio_venta, stock, descripcion,
            peso, tamano, unidad_medida, presentacion, codigo, activo,
            fecha_vencimiento, marca_id
         FROM productos 
         WHERE fecha_vencimiento IS NOT NULL 
         AND fecha_vencimiento <= date('now', '+' || ?1 || ' days')
         AND activo = 1
         ORDER BY fecha_vencimiento"
    )?;
    
    let productos = stmt.query_map(params![dias], mapear_producto)?;
    productos.collect()
}

/// Obtener productos con stock bajo
pub fn obtener_productos_stock_bajo(conn: &Connection, minimo: i64) -> Result<Vec<Producto>> {
    let mut stmt = conn.prepare(
        "SELECT 
            id, nombre, precio_neto, precio_venta, stock, descripcion,
            peso, tamano, unidad_medida, presentacion, codigo, activo,
            fecha_vencimiento, marca_id
         FROM productos 
         WHERE stock <= ?1 AND activo = 1
         ORDER BY stock ASC"
    )?;
    
    let productos = stmt.query_map(params![minimo], mapear_producto)?;
    productos.collect()
}
