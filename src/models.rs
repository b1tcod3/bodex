use chrono::{NaiveDate, NaiveDateTime};

// ==========================================
// ENUMS Y ROLES
// ==========================================

#[derive(Debug, Clone, PartialEq)]
pub enum Rol {
    Admin,
    Operador,
    Vendedor,
}

impl From<String> for Rol {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Admin" => Rol::Admin,
            "Operador" => Rol::Operador,
            _ => Rol::Vendedor,
        }
    }
}

impl ToString for Rol {
    fn to_string(&self) -> String {
        match self {
            Rol::Admin => "Admin".to_string(),
            Rol::Operador => "Operador".to_string(),
            Rol::Vendedor => "Vendedor".to_string(),
        }
    }
}

// ==========================================
// USUARIOS
// ==========================================

pub struct Usuario {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub rol: Rol,
    pub activo: bool,
    pub ultimo_login: Option<NaiveDateTime>,
}

// ==========================================
// MARCAS
// ==========================================

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

// ==========================================
// PRODUCTOS
// ==========================================

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

// ==========================================
// VENTAS
// ==========================================

#[derive(Debug, Clone)]
pub struct Venta {
    pub id: i64,
    pub fecha: NaiveDateTime,
    pub total: f64,
    pub usuario_id: Option<i64>,        // Para saber quién vendió
    pub cliente_nombre: Option<String>,  // Para saber a quién se vendió
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
