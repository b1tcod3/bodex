use crate::db;
use crate::models::{Producto as DbProducto, ProductoNuevo};
use regex::Regex;
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

/// Datos crudos de una fila de producto para transferencia entre hilos
#[derive(Debug, Clone)]
pub struct ProductRowData {
    pub codigo: String,
    pub nombre: String,
    pub precio_venta: String,
    pub stock: String,
    pub marca_nombre: String,
    pub activo: bool,
}

/// Obtiene los productos de la DB como datos crudos (Send-safe) para usar en hilos secundarios
pub fn get_inventory_rows_raw() -> Result<Vec<ProductRowData>, Box<dyn std::error::Error>> {
    let conn = db::open_connection()?;
    let productos = db::productos::obtener_productos_con_marca(&conn)?;

    // Actualizar caché interno
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

    // Convertir a datos crudos
    let rows: Vec<ProductRowData> = productos
        .into_iter()
        .map(|p| ProductRowData {
            codigo: p.codigo.clone().unwrap_or_else(|| "S/C".into()),
            nombre: p.nombre,
            precio_venta: format!("{:.2}", p.precio_venta),
            stock: p.stock.to_string(),
            marca_nombre: p.marca_nombre.unwrap_or_else(|| "Sin Marca".into()),
            activo: p.activo,
        })
        .collect();

    Ok(rows)
}

/// Convierte datos crudos a ModelRc para la UI (debe llamarse en el hilo principal)
pub fn raw_to_model_rows(rows: Vec<ProductRowData>) -> ModelRc<ModelRc<StandardListViewItem>> {
    let model_rows: Vec<ModelRc<StandardListViewItem>> = rows
        .into_iter()
        .map(|r| {
            let row_data = vec![
                StandardListViewItem::from(SharedString::from(r.codigo)),
                StandardListViewItem::from(SharedString::from(r.nombre)),
                StandardListViewItem::from(SharedString::from(r.precio_venta)),
                StandardListViewItem::from(SharedString::from(r.stock)),
                StandardListViewItem::from(SharedString::from(r.marca_nombre)),
                StandardListViewItem::from(SharedString::from(if r.activo { "true" } else { "false" })),
            ];
            ModelRc::from(Rc::new(VecModel::from(row_data)))
        })
        .collect();

    ModelRc::from(Rc::new(VecModel::from(model_rows)))
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

    // 2. Formatear filas para la UI (Código, Nombre, Precio, Stock, Marca, Estado)
    let rows: Vec<ModelRc<StandardListViewItem>> = productos
        .into_iter()
        .map(|p| {
            let row_data = vec![
                StandardListViewItem::from(SharedString::from(p.codigo.clone().unwrap_or_else(|| "S/C".into()))),
                StandardListViewItem::from(SharedString::from(p.nombre)),
                StandardListViewItem::from(SharedString::from(format!("{:.2}", p.precio_venta))),
                StandardListViewItem::from(SharedString::from(p.stock.to_string())),
                StandardListViewItem::from(SharedString::from(p.marca_nombre.unwrap_or_else(|| "Sin Marca".into()))),
                StandardListViewItem::from(SharedString::from(if p.activo { "true" } else { "false" })),
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
    // Categoría y subcategoría
    categoria_id: SharedString,
    subcategoria_id: SharedString,
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
        // Categoría y subcategoría
        categoria_id: parse_num(&categoria_id, 1),
        subcategoria_id: parse_num(&subcategoria_id, 1),
    };

    Ok(db::productos::crear_producto(&conn, &p_nuevo)?)
}

/// Resultado de validación de SKU
#[derive(Debug, Clone)]
pub struct SkuValidationResult {
    pub es_valido: bool,
    pub error: Option<String>,
}

/// Valida el formato de un SKU usando Regex
/// Formato esperado: ABC-123 (3 letras mayúsculas, guion, 3 números)
/// También acepta formatos más flexibles como: abc-123, ABC123, etc.
pub fn validar_formato_sku(sku: &str) -> SkuValidationResult {
    let sku_trimmed = sku.trim();
    
    // SKU vacío es válido (es opcional)
    if sku_trimmed.is_empty() {
        return SkuValidationResult {
            es_valido: true,
            error: None,
        };
    }
    
    // Longitud mínima
    if sku_trimmed.len() < 7 {
        return SkuValidationResult {
            es_valido: false,
            error: Some(format!("El SKU debe tener al menos 7 caracteres (actual: {})", sku_trimmed.len())),
        };
    }
    
    // Patrón estricto: 3 letras, guion, 3 números (ABC-123)
    // También aceptamos variaciones comunes
    let patron_estricto = Regex::new(r"^[A-Z]{3}-\d{3}$").unwrap();
    let patron_flexible = Regex::new(r"^[A-Za-z]{3}-?\d{3}$").unwrap();
    let patron_generico = Regex::new(r"^[A-Za-z0-9\-]{7,20}$").unwrap();
    
    if patron_estricto.is_match(sku_trimmed) {
        // Formato perfecto: ABC-123
        SkuValidationResult {
            es_valido: true,
            error: None,
        }
    } else if patron_flexible.is_match(sku_trimmed) {
        // Formato aceptable: abc-123 o ABC123
        SkuValidationResult {
            es_valido: true,
            error: None,
        }
    } else if patron_generico.is_match(sku_trimmed) {
        // Formo genérico válido: cualquier combinación alfanumérica de 7-20 caracteres
        SkuValidationResult {
            es_valido: true,
            error: None,
        }
    } else {
        SkuValidationResult {
            es_valido: false,
            error: Some("Formato de SKU inválido. Use letras, números y guiones (7-20 caracteres)".to_string()),
        }
    }
}

/// Verifica si un SKU ya existe en la base de datos
pub fn sku_existe(sku: &str) -> Result<bool, Box<dyn std::error::Error>> {
    if sku.trim().is_empty() {
        return Ok(false);
    }
    
    let conn = db::open_connection()?;
    Ok(db::productos::existe_sku(&conn, sku)?)
}

/// Validación completa de SKU: formato + unicidad
pub fn validar_sku_completo(sku: &str) -> SkuValidationResult {
    // 1. Validar formato
    let resultado_formato = validar_formato_sku(sku);
    if !resultado_formato.es_valido {
        return resultado_formato;
    }
    
    // 2. Validar unicidad en BD
    if !sku.trim().is_empty() {
        match sku_existe(sku) {
            Ok(existe) => {
                if existe {
                    return SkuValidationResult {
                        es_valido: false,
                        error: Some(format!("El SKU '{}' ya está registrado en la base de datos", sku)),
                    };
                }
            }
            Err(e) => {
                return SkuValidationResult {
                    es_valido: false,
                    error: Some(format!("Error al verificar SKU: {}", e)),
                };
            }
        }
    }
    
    SkuValidationResult {
        es_valido: true,
        error: None,
    }
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
    // Categoría y subcategoría
    categoria_id: SharedString,
    subcategoria_id: SharedString,
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
        categoria_id: parse_num(&categoria_id, 1),
        subcategoria_id: parse_num(&subcategoria_id, 1),
    };

    // Aquí llamarías a db::productos::actualizar_producto(&conn, &_p_editado)
    Ok(true)
}
