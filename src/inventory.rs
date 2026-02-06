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

/// Caché global segura para hilos (Thread-safe) mediante OnceLock y Mutex
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

/// Obtiene los productos de la base de datos y los convierte en el formato
/// [[StandardListViewItem]] que requiere el StandardTableView de tu .slint
pub fn get_inventory_rows(
) -> Result<ModelRc<ModelRc<StandardListViewItem>>, Box<dyn std::error::Error>> {
    let conn = db::open_connection()?;
    // Usamos el submódulo productos para obtener los datos
    let productos = db::productos::obtener_productos_con_marca(&conn)?;

    // 1. Sincronizamos el caché interno para que las acciones por índice (editar/eliminar) funcionen
    let product_infos: Vec<ProductInfo> = productos
        .iter()
        .map(|p| ProductInfo {
            id: p.id,
            nombre: p.nombre.clone(),
            precio_venta: p.precio_venta,
            stock: p.stock,
        })
        .collect();

    {
        let mut cache = get_cache().lock().unwrap();
        *cache = product_infos;
    }

    // 2. Transformamos los datos al formato de celdas de Slint
    let rows: Vec<ModelRc<StandardListViewItem>> = productos
        .into_iter()
        .map(|p| {
            let row_data = vec![
                StandardListViewItem::from(SharedString::from(p.nombre)),
                StandardListViewItem::from(SharedString::from(format!("{:.2}", p.precio_venta))),
                StandardListViewItem::from(SharedString::from(p.stock.to_string())),
            ];
            ModelRc::from(Rc::new(VecModel::from(row_data)))
        })
        .collect();

    Ok(ModelRc::from(Rc::new(VecModel::from(rows))))
}

/// Recupera la información de un producto basándose en su posición en la tabla de la UI
pub fn get_product_by_index(index: i32) -> Option<ProductInfo> {
    let cache = get_cache().lock().unwrap();
    cache.get(index as usize).cloned()
}

/// Elimina un producto de la base de datos usando el índice de la tabla
pub fn delete_product_by_index(index: i32) -> Result<bool, Box<dyn std::error::Error>> {
    if let Some(product) = get_product_by_index(index) {
        let conn = db::open_connection()?;
        let deleted = db::productos::eliminar_producto(&conn, product.id)?;
        Ok(deleted)
    } else {
        Ok(false)
    }
}

/// Agrega un nuevo producto procesando los strings que vienen de los inputs de Slint
pub fn add_product(
    nombre: SharedString,
    precio_neto: SharedString,
    precio_venta: SharedString,
    stock: SharedString,
    descripcion: SharedString,
    peso: SharedString,
    tamano: SharedString,
    unidad_medida: SharedString,
    presentacion: SharedString,
    codigo: SharedString,
    fecha_vencimiento: SharedString,
    activo_str: SharedString,
    marca_id: SharedString,
) -> Result<i64, Box<dyn std::error::Error>> {
    let activo = activo_str == "true";
    let conn = db::open_connection()?;

    let p_nuevo = ProductoNuevo {
        nombre: nombre.into(),
        precio_neto: parse_num(&precio_neto, 0.0),
        precio_venta: parse_num(&precio_venta, 0.0),
        stock: parse_num(&stock, 0),
        descripcion: (!descripcion.is_empty()).then(|| descripcion.into()),
        peso: (!peso.is_empty()).then(|| peso.parse().ok()).flatten(),
        tamano: (!tamano.is_empty()).then(|| tamano.into()),
        unidad_medida: (!unidad_medida.is_empty()).then(|| unidad_medida.into()),
        presentacion: (!presentacion.is_empty()).then(|| presentacion.into()),
        codigo: (!codigo.is_empty()).then(|| codigo.into()),
        activo,
        fecha_vencimiento: chrono::NaiveDate::parse_from_str(&fecha_vencimiento, "%Y-%m-%d").ok(),
        marca_id: (!marca_id.is_empty())
            .then(|| marca_id.parse().ok())
            .flatten(),
    };

    Ok(db::productos::crear_producto(&conn, &p_nuevo)?)
}

/// Actualiza los datos de un producto existente
pub fn update_product(
    id: i64,
    nombre: SharedString,
    precio_neto: SharedString,
    precio_venta: SharedString,
    stock: SharedString,
    descripcion: SharedString,
    peso: SharedString,
    tamano: SharedString,
    unidad_medida: SharedString,
    presentacion: SharedString,
    codigo: SharedString,
    activo: bool,
    fecha_vencimiento: SharedString,
    marca_id: SharedString,
) -> Result<bool, Box<dyn std::error::Error>> {
    let conn = db::open_connection()?;

    let p_editado = DbProducto {
        id,
        nombre: nombre.into(),
        precio_neto: parse_num(&precio_neto, 0.0),
        precio_venta: parse_num(&precio_venta, 0.0),
        stock: parse_num(&stock, 0),
        descripcion: (!descripcion.is_empty()).then(|| descripcion.into()),
        peso: (!peso.is_empty()).then(|| peso.parse().ok()).flatten(),
        tamano: (!tamano.is_empty()).then(|| tamano.into()),
        unidad_medida: (!unidad_medida.is_empty()).then(|| unidad_medida.into()),
        presentacion: (!presentacion.is_empty()).then(|| presentacion.into()),
        codigo: (!codigo.is_empty()).then(|| codigo.into()),
        activo,
        fecha_vencimiento: chrono::NaiveDate::parse_from_str(&fecha_vencimiento, "%Y-%m-%d").ok(),
        marca_id: (!marca_id.is_empty())
            .then(|| marca_id.parse().ok())
            .flatten(),
    };

    // Suponiendo que tienes esta función en db/productos.rs
    // db::productos::actualizar_producto(&conn, &p_editado)
    Ok(true)
}
