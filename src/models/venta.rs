use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct Venta {
    pub id: i64,
    pub fecha: NaiveDateTime,
    pub total: f64,
    pub usuario_id: Option<i64>,        // Para saber quién vendió
    pub cliente_nombre: Option<String>, // Para saber a quién se vendió
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
