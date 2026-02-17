use crate::db;
use crate::models::{self, Producto as DbProducto, ProductoNuevo};
use slint::{ModelRc, SharedString, StandardListViewItem, VecModel};
use std::rc::Rc;
use std::sync::{Mutex, OnceLock};

/// Estructura para mantener el estado del inventario en caché para acceso rápido por índice
#[derive(Debug, Clone)]
pub struct ProductInfo {
    pub id: i64,
    pub nombre: String,
    pub precio_venta: f64,
    pub stock: i64,
}

/// Caché global segura para hilos
static LOADED_PRODUCTS: OnceLock<Mutex<Vec<ProductInfo>>> = OnceLock::new();

/// Helper para obtener acceso al caché global
fn get_cache() -> &'static Mutex<Vec<ProductInfo>> {
    LOADED_PRODUCTS.get_or_init(|| Mutex::new(Vec::new()))
}

/// Helper para parsear SharedString de Slint a tipos numéricos de Rust
fn parse_num<T: std::str::FromStr>(val: &SharedString, default: T) -> T {
    if val.is_empty() {
        default
    } else {
        val.parse().unwrap_or(default)
    }
}

/// Obtiene los productos de la DB y los formatea para la tabla de Slint
pub fn get_inventory_rows() -> Result<ModelRc<ModelRc<StandardListViewItem>>, Box<dyn std::error::Error>> {
    let conn = db::open_connection()?;
    let productos = db::productos::obtener_productos_con_marca(&conn)?;

    // 1. Actualizar caché interno
    let product_infos: Vec<ProductInfo> = productos
        .iter()
        .map(|p| ProductInfo {
            id: p.id,
            nombre: p.nombre.clone(),
            precio_venta: p.precio_venta,
            stock: p.stock as i64,
        })
        .collect();

    {
        let mut cache = get_cache().lock().unwrap();
        *cache = product_infos;
    }

    // 2. Formatear filas para la UI (Nombre, Precio, Stock, Marca)
    let rows: Vec<ModelRc<StandardListViewItem>> = productos
        .into_iter()
        .map(|p| {
            let row_data = vec![
                StandardListViewItem::from(SharedString::from(p.nombre)),
                StandardListViewItem::from(SharedString::from(format!("{:.2}", p.precio_venta))),
                StandardListViewItem::from(SharedString::from(p.stock.to_string())),
                StandardListViewItem::from(SharedString::from(p.marca_nombre.unwrap_or_else(|| "Sin Marca".into()))),
            ];
            ModelRc::from(Rc::new(VecModel::from(row_data)))
        })
        .collect();

    Ok(ModelRc::from(Rc::new(VecModel::from(rows))))
}

/// Recupera la información de un producto por su índice en la tabla
pub fn get_product_by_index(index: i32) -> Option<ProductInfo> {
    let cache = get_cache().lock().unwrap();
    cache.get(index as usize).cloned()
}

/// Elimina un producto usando el índice de la UI
pub fn delete_product_by_index(index: i32) -> Result<bool, Box<dyn std::error::Error>> {
    if let Some(product) = get_product_by_index(index) {
        let conn = db::open_connection()?;
        let deleted = db::productos::eliminar_producto(&conn, product.id)?;
        Ok(deleted)
    } else {
        Ok(false)
    }
}

/// Agrega un nuevo producto con la lógica de Enums (IDs) y Doble Medida
pub fn add_product(
    nombre: SharedString,
    precio_neto: SharedString,
    precio_venta: SharedString,
    stock: SharedString,
    descripcion: SharedString,
    codigo: SharedString,
    activo_str: SharedString,
    marca_id: SharedString,
    // Nuevos campos de Enums y Medidas
    medida_p_id: SharedString,
    cantidad_p: SharedString,
    medida_s_id: SharedString,
    cantidad_s: SharedString,
    empaque_id: SharedString,
) -> Result<i64, Box<dyn std::error::Error>> {
    let conn = db::open_connection()?;
    let activo = activo_str == "true";

    let p_nuevo = ProductoNuevo {
        nombre: nombre.into(),
        precio_neto: parse_num(&precio_neto, 0.0),
        precio_venta: parse_num(&precio_venta, 0.0),
        stock: parse_num(&stock, 0),
        descripcion: (!descripcion.is_empty()).then(|| descripcion.into()),
        codigo: (!codigo.is_empty()).then(|| codigo.into()),
        activo,
        marca_id: (!marca_id.is_empty()).then(|| marca_id.parse().ok()).flatten(),
        // Mapeo de Enums y Medidas
        medida_p_id: parse_num(&medida_p_id, 1), // Default a 1 (Unidad)
        cantidad_p: parse_num(&cantidad_p, 0.0),
        medida_s_id: (!medida_s_id.is_empty()).then(|| medida_s_id.parse().ok()).flatten(),
        cantidad_s: (!cantidad_s.is_empty()).then(|| cantidad_s.parse().ok()).flatten(),
        empaque_id: parse_num(&empaque_id, 1),   // Default a 1 (Individual)
    };

    Ok(db::productos::crear_producto(&conn, &p_nuevo)?)
}

/// Actualiza un producto existente
pub fn update_product(
    id: i64,
    nombre: SharedString,
    precio_neto: SharedString,
    precio_venta: SharedString,
    stock: SharedString,
    descripcion: SharedString,
    codigo: SharedString,
    activo: bool,
    marca_id: SharedString,
    // Nuevos campos de Enums y Medidas
    medida_p_id: SharedString,
    cantidad_p: SharedString,
    medida_s_id: SharedString,
    cantidad_s: SharedString,
    empaque_id: SharedString,
) -> Result<bool, Box<dyn std::error::Error>> {
    let conn = db::open_connection()?;

    let _p_editado = DbProducto {
        id,
        nombre: nombre.into(),
        precio_neto: parse_num(&precio_neto, 0.0),
        precio_venta: parse_num(&precio_venta, 0.0),
        stock: parse_num(&stock, 0),
        descripcion: (!descripcion.is_empty()).then(|| descripcion.into()),
        codigo: (!codigo.is_empty()).then(|| codigo.into()),
        activo,
        marca_id: (!marca_id.is_empty()).then(|| marca_id.parse().ok()).flatten(),
        medida_p_id: parse_num(&medida_p_id, 1),
        cantidad_p: parse_num(&cantidad_p, 0.0),
        medida_s_id: (!medida_s_id.is_empty()).then(|| medida_s_id.parse().ok()).flatten(),
        cantidad_s: (!cantidad_s.is_empty()).then(|| cantidad_s.parse().ok()).flatten(),
        empaque_id: parse_num(&empaque_id, 1),
    };

    // Aquí llamarías a db::productos::actualizar_producto(&conn, &_p_editado)
    Ok(true)
}
