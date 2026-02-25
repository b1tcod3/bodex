use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct Producto {
    pub id: i64,
    pub nombre: String,
    pub precio_neto: f64,
    pub precio_venta: f64,
    pub stock: i64,
    pub descripcion: Option<String>,
    pub codigo: Option<String>,
    pub activo: bool,
    pub marca_id: Option<i64>,
    // Nuevos campos técnicos
    pub medida_p_id: i32, // ID del Enum Medida (Principal)
    pub cantidad_p: f64,
    pub medida_s_id: Option<i32>, // ID del Enum Medida (Secundaria/Opcional)
    pub cantidad_s: Option<f64>,
    pub empaque_id: i32, // ID del Enum TipoEmpaque
    // Categoría y subcategoría
    pub categoria_id: i32,
    pub subcategoria_id: i32,
}

#[derive(Debug, Clone)]
pub struct ProductoNuevo {
    pub nombre: String,
    pub precio_neto: f64,
    pub precio_venta: f64,
    pub stock: i64,
    pub descripcion: Option<String>,
    pub codigo: Option<String>,
    pub activo: bool,
    pub marca_id: Option<i64>,
    pub medida_p_id: i32,
    pub cantidad_p: f64,
    pub medida_s_id: Option<i32>,
    pub cantidad_s: Option<f64>,
    pub empaque_id: i32,
    // Categoría y subcategoría
    pub categoria_id: i32,
    pub subcategoria_id: i32,
}

#[derive(Debug, Clone)]
pub struct ProductoConMarca {
    pub id: i64,
    pub nombre: String,
    pub precio_neto: f64,
    pub precio_venta: f64,
    pub stock: i64,
    pub descripcion: Option<String>,
    pub codigo: Option<String>,
    pub activo: bool,
    pub marca_id: Option<i64>,
    pub marca_nombre: Option<String>,
    // Campos de visualización rápida
    pub medida_p_id: i32,
    pub cantidad_p: f64,
    pub empaque_id: i32,
    // Categoría y subcategoría
    pub categoria_id: i32,
    pub subcategoria_id: i32,
}
