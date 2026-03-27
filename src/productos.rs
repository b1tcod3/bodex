use crate::db;
use crate::models::producto::{ProductoConMarca, ProductoNuevo};
use crate::inventory::ProductRowData;
use chrono::NaiveDateTime;
use std::sync::OnceLock;
use std::sync::Mutex;
use slint::SharedString;

/// Información básica de un producto para caché rápida
#[derive(Debug, Clone)]
pub struct ProductInfo {
    pub id: i64,
    pub nombre: String,
    pub precio_venta: f64,
    pub stock: f64,
    pub activo: bool,
}

/// Caché global de productos indexados por ID
static LOADED_PRODUCTS: OnceLock<Mutex<Vec<ProductInfo>>> = OnceLock::new();

/// Obtiene la caché de productos
fn get_cache() -> &'static Mutex<Vec<ProductInfo>> {
    LOADED_PRODUCTS.get_or_init(|| Mutex::new(Vec::new()))
}

/// Helper para parsear valores numéricos de SharedString
fn parse_num<T: std::str::FromStr>(val: &SharedString, default: T) -> T {
    val.trim().parse().unwrap_or(default)
}

/// Resultado de validación de SKU
#[derive(Debug)]
pub struct SkuValidationResult {
    pub es_valido: bool,
    pub error: Option<String>,
}

/// Valida el formato del SKU usando expresiones regulares
pub fn validar_formato_sku(sku: &str) -> SkuValidationResult {
    let sku = sku.trim();
    
    // Validar longitud (máximo 20 caracteres)
    if sku.len() > 20 {
        return SkuValidationResult {
            es_valido: false,
            error: Some("El SKU no debe exceder los 20 caracteres".to_string()),
        };
    }
    
    // Validar que no esté vacío
    if sku.is_empty() {
        return SkuValidationResult {
            es_valido: false,
            error: Some("El SKU no puede estar vacío".to_string()),
        };
    }
    
    // Validar formato: solo letras, números, guiones y guiones bajos
    let re = regex::Regex::new(r"^[A-Za-z0-9_-]+$").unwrap();
    if !re.is_match(sku) {
        return SkuValidationResult {
            es_valido: false,
            error: Some("El SKU solo puede contener letras, números, guiones y guiones bajos".to_string()),
        };
    }
    
    SkuValidationResult {
        es_valido: true,
        error: None,
    }
}

/// Verifica si un SKU ya existe en la base de datos
pub fn sku_existe(sku: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let conn = db::open_connection()?;
    let result = db::productos::existe_sku(&conn, sku)?;
    Ok(result)
}

/// Validación completa de SKU (formato + unicidad)
pub fn validar_sku_completo(sku: &str) -> SkuValidationResult {
    // Primero validar formato
    let validacion_formato = validar_formato_sku(sku);
    if !validacion_formato.es_valido {
        return validacion_formato;
    }
    
    // Luego validar unicidad
    match sku_existe(sku) {
        Ok(existe) => {
            if existe {
                SkuValidationResult {
                    es_valido: false,
                    error: Some("El código SKU ya existe en la base de datos".to_string()),
                }
            } else {
                SkuValidationResult {
                    es_valido: true,
                    error: None,
                }
            }
        }
        Err(e) => SkuValidationResult {
            es_valido: false,
            error: Some(format!("Error al verificar SKU: {}", e)),
        },
    }
}

/// Obtiene un producto por su índice en la caché
pub fn get_product_by_index(index: i32) -> Option<ProductInfo> {
    let cache = get_cache();
    let productos = cache.lock().unwrap();
    
    if index >= 0 && (index as usize) < productos.len() {
        Some(productos[index as usize].clone())
    } else {
        None
    }
}

/// Elimina un producto por su índice en la caché
pub fn delete_product_by_index(index: i32) -> Result<bool, Box<dyn std::error::Error>> {
    let conn = db::open_connection()?;
    
    // Obtener el producto para saber su ID
    if let Some(product) = get_product_by_index(index) {
        let result = db::productos::eliminar_producto(&conn, product.id)?;
        
        if result {
            // Actualizar caché
            let cache = get_cache();
            let mut productos = cache.lock().unwrap();
            if index >= 0 && (index as usize) < productos.len() {
                productos.remove(index as usize);
            }
        }
        
        Ok(result)
    } else {
        Ok(false)
    }
}

/// Crea un nuevo producto
pub fn add_product(
    nombre: SharedString,
    precio_neto: SharedString,
    precio_venta: SharedString,
    stock: SharedString,
    descripcion: SharedString,
    codigo: SharedString,
    activo_str: SharedString,
    marca_id: SharedString,
    medida_p_id: SharedString,
    cantidad_p: SharedString,
    medida_s_id: SharedString,
    cantidad_s: SharedString,
    empaque_id: SharedString,
    categoria_id: SharedString,
    subcategoria_id: SharedString,
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = db::open_connection()?;
    
    // Validar SKU
    let validacion_sku = validar_sku_completo(codigo.as_str());
    if !validacion_sku.es_valido {
        return Err(format!("SKU inválido: {}", validacion_sku.error.unwrap_or("Error desconocido".to_string())).into());
    }
    
    // Validar campos requeridos
    if nombre.trim().is_empty() {
        return Err("El nombre del producto es requerido".into());
    }
    
    // Parseo seguro de valores numéricos
    let neto: f64 = precio_neto.as_str().trim().parse().unwrap_or(-1.0);
    let venta: f64 = precio_venta.as_str().trim().parse().unwrap_or(-1.0);
    let stock_val: f64 = stock.as_str().trim().parse().unwrap_or(0.0);
    let medida_p: i32 = medida_p_id.as_str().trim().parse().unwrap_or(1);
    let cantidad_p_val: f64 = cantidad_p.as_str().trim().parse().unwrap_or(1.0);
    let medida_s: i32 = medida_s_id.as_str().trim().parse().unwrap_or(1);
    let cantidad_s_val: f64 = cantidad_s.as_str().trim().parse().unwrap_or(1.0);
    let marca: i64 = marca_id.as_str().trim().parse().unwrap_or(1);
    let empaque: i32 = empaque_id.as_str().trim().parse().unwrap_or(1);
    let categoria: i32 = categoria_id.as_str().trim().parse().unwrap_or(1);
    let subcategoria: i32 = subcategoria_id.as_str().trim().parse().unwrap_or(1);
    
    // Validar que los precios sean positivos
    if neto <= 0.0 {
        return Err("El costo neto debe ser un número positivo mayor a 0".into());
    }
    
    if venta <= 0.0 {
        return Err("El precio de venta debe ser un número positivo mayor a 0".into());
    }
    
    // Validar que no haya pérdida (precio de venta >= costo)
    if venta < neto {
        return Err(format!(
            "¡Pérdida detectada! El precio de venta (${:.2}) es menor al costo (${:.2})",
            venta, neto
        ).into());
    }
    
    // Crear el producto
    let producto = ProductoNuevo {
        nombre: nombre.to_string(),
        precio_neto: neto,
        precio_venta: venta,
        stock: stock_val as i64,
        descripcion: if descripcion.trim().is_empty() { None } else { Some(descripcion.to_string()) },
        codigo: Some(codigo.to_string()),
        activo: activo_str == "true",
        marca_id: Some(marca),
        medida_p_id: medida_p,
        cantidad_p: cantidad_p_val,
        medida_s_id: Some(medida_s),
        cantidad_s: Some(cantidad_s_val),
        empaque_id: empaque,
        categoria_id: categoria,
        subcategoria_id: subcategoria,
    };
    
    let id = db::productos::crear_producto(&conn, &producto)?;
    
    // Actualizar caché
    let nuevo_producto = ProductInfo {
        id,
        nombre: producto.nombre,
        precio_venta: producto.precio_venta,
        stock: producto.stock as f64,
        activo: producto.activo,
    };
    
    let cache = get_cache();
    let mut productos = cache.lock().unwrap();
    productos.push(nuevo_producto);
    
    Ok(())
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
    activo_str: SharedString,
    marca_id: SharedString,
    medida_p_id: SharedString,
    cantidad_p: SharedString,
    medida_s_id: SharedString,
    cantidad_s: SharedString,
    empaque_id: SharedString,
    categoria_id: SharedString,
    subcategoria_id: SharedString,
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = db::open_connection()?;
    
    // Validar SKU si cambió
    if !codigo.trim().is_empty() {
        let validacion_sku = validar_sku_completo(codigo.as_str());
        if !validacion_sku.es_valido {
            return Err(format!("SKU inválido: {}", validacion_sku.error.unwrap_or("Error desconocido".to_string())).into());
        }
    }
    
    // Validar campos requeridos
    if nombre.trim().is_empty() {
        return Err("El nombre del producto es requerido".into());
    }
    
    // Parseo seguro de valores numéricos
    let neto: f64 = precio_neto.as_str().trim().parse().unwrap_or(-1.0);
    let venta: f64 = precio_venta.as_str().trim().parse().unwrap_or(-1.0);
    let stock_val: f64 = stock.as_str().trim().parse().unwrap_or(0.0);
    let medida_p: i32 = medida_p_id.as_str().trim().parse().unwrap_or(1);
    let cantidad_p_val: f64 = cantidad_p.as_str().trim().parse().unwrap_or(1.0);
    let medida_s: i32 = medida_s_id.as_str().trim().parse().unwrap_or(1);
    let cantidad_s_val: f64 = cantidad_s.as_str().trim().parse().unwrap_or(1.0);
    let marca: i64 = marca_id.as_str().trim().parse().unwrap_or(1);
    let empaque: i32 = empaque_id.as_str().trim().parse().unwrap_or(1);
    let categoria: i32 = categoria_id.as_str().trim().parse().unwrap_or(1);
    let subcategoria: i32 = subcategoria_id.as_str().trim().parse().unwrap_or(1);
    
    // Validar que los precios sean positivos
    if neto <= 0.0 {
        return Err("El costo neto debe ser un número positivo mayor a 0".into());
    }
    
    if venta <= 0.0 {
        return Err("El precio de venta debe ser un número positivo mayor a 0".into());
    }
    
    // Validar que no haya pérdida (precio de venta >= costo)
    if venta < neto {
        return Err(format!(
            "¡Pérdida detectada! El precio de venta (${:.2}) es menor al costo (${:.2})",
            venta, neto
        ).into());
    }
    
    // Crear el producto actualizado
    let producto = ProductoNuevo {
        nombre: nombre.to_string(),
        precio_neto: neto,
        precio_venta: venta,
        stock: stock_val as i64,
        descripcion: if descripcion.trim().is_empty() { None } else { Some(descripcion.to_string()) },
        codigo: Some(codigo.to_string()),
        activo: activo_str == "true",
        marca_id: Some(marca),
        medida_p_id: medida_p,
        cantidad_p: cantidad_p_val,
        medida_s_id: Some(medida_s),
        cantidad_s: Some(cantidad_s_val),
        empaque_id: empaque,
        categoria_id: categoria,
        subcategoria_id: subcategoria,
    };
    
    // Actualizar en la base de datos
    // TODO: Implementar db::productos::actualizar_producto
    // db::productos::actualizar_producto(&conn, id, &producto)?;
    
    // Actualizar caché
    let cache = get_cache();
    let mut productos = cache.lock().unwrap();
    for p in productos.iter_mut() {
        if p.id == id {
            p.nombre = producto.nombre;
            p.precio_venta = producto.precio_venta;
            p.stock = producto.stock as f64;
            p.activo = producto.activo;
            break;
        }
    }
    
    Ok(())
}

/// Actualiza la caché de productos desde datos raw
pub fn update_cache(productos_raw: Vec<ProductRowData>) {
    let cache = get_cache();
    let mut cache_guard = cache.lock().unwrap();
    cache_guard.clear();
    
    for producto in productos_raw {
        cache_guard.push(ProductInfo {
            id: 0, // No tenemos ID en ProductRowData, usar 0 como placeholder
            nombre: producto.nombre,
            precio_venta: producto.precio_venta.parse().unwrap_or(0.0),
            stock: producto.stock.parse().unwrap_or(0.0),
            activo: producto.activo,
        });
    }
}

/// Actualiza la caché de productos desde ProductInfo
pub fn update_cache_from_info(productos_info: Vec<ProductInfo>) {
    let cache = get_cache();
    let mut cache_guard = cache.lock().unwrap();
    cache_guard.clear();
    
    for producto in productos_info {
        cache_guard.push(producto);
    }
}
