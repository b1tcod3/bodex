use crate::db;
use slint::{ModelRc, StandardListViewItem, VecModel};
use std::rc::Rc;

/// Estructura para mantener el estado del inventario con IDs
#[derive(Debug, Clone)]
pub struct ProductInfo {
    pub id: i64,
    pub nombre: String,
    pub precio_venta: f64,
    pub stock: i64,
    pub codigo: Option<String>,
    pub marca_nombre: Option<String>,
}

/// Producto representado como fila para la UI
pub type InventoryRow = ModelRc<StandardListViewItem>;

/// Almacena los productos cargados para poder acceder a ellos por índice
static mut LOADED_PRODUCTS: Vec<ProductInfo> = Vec::new();

/// Obtiene los productos de la base de datos y los convierte en filas para la tabla
pub fn get_inventory_rows() -> Result<ModelRc<InventoryRow>, Box<dyn std::error::Error>> {
    let conn = db::open_connection()?;
    let productos = db::obtener_productos_con_marca(&conn)?;

    // Guardar productos para acceso por índice
    let product_infos: Vec<ProductInfo> = productos
        .iter()
        .map(|p| ProductInfo {
            id: p.id,
            nombre: p.nombre.clone(),
            precio_venta: p.precio_venta,
            stock: p.stock,
            codigo: p.codigo.clone(),
            marca_nombre: p.marca_nombre.clone(),
        })
        .collect();

    unsafe {
        LOADED_PRODUCTS = product_infos;
    }

    let rows: Vec<InventoryRow> = productos
        .into_iter()
        .map(|p| {
            let codigo_str = p.codigo.unwrap_or_else(|| "-".to_string());
            let marca_str = p.marca_nombre.unwrap_or_else(|| "-".to_string());
            let estado_str = if p.activo { "Activo" } else { "Inactivo" };
            
            ModelRc::from(Rc::new(VecModel::from(vec![
                StandardListViewItem::from(slint::SharedString::from(codigo_str)),
                StandardListViewItem::from(slint::SharedString::from(p.nombre)),
                StandardListViewItem::from(slint::SharedString::from(format!(
                    "${:.2}",
                    p.precio_venta
                ))),
                StandardListViewItem::from(slint::SharedString::from(p.stock.to_string())),
                StandardListViewItem::from(slint::SharedString::from(marca_str)),
                StandardListViewItem::from(slint::SharedString::from(estado_str)),
            ])))
        })
        .collect();

    Ok(ModelRc::from(Rc::new(VecModel::from(rows))))
}

/// Obtiene un producto por su índice en la tabla
pub fn get_product_by_index(index: i32) -> Option<ProductInfo> {
    unsafe {
        if index >= 0 && (index as usize) < LOADED_PRODUCTS.len() {
            Some(LOADED_PRODUCTS[index as usize].clone())
        } else {
            None
        }
    }
}

/// Elimina un producto por su índice en la tabla
pub fn delete_product_by_index(index: i32) -> Result<bool, Box<dyn std::error::Error>> {
    if let Some(product) = get_product_by_index(index) {
        let conn = db::open_connection()?;
        let deleted = db::eliminar_producto(&conn, product.id)?;
        Ok(deleted)
    } else {
        Ok(false)
    }
}

/// Actualiza un producto existente
pub fn update_product(
    id: i64,
    nombre: slint::SharedString,
    precio_neto: slint::SharedString,
    precio_venta: slint::SharedString,
    stock: slint::SharedString,
    descripcion: slint::SharedString,
    peso: slint::SharedString,
    tamano: slint::SharedString,
    unidad_medida: slint::SharedString,
    presentacion: slint::SharedString,
    codigo: slint::SharedString,
    activo: bool,
    fecha_vencimiento: slint::SharedString,
    marca_id: slint::SharedString,
) -> Result<bool, Box<dyn std::error::Error>> {
    let conn = db::open_connection()?;
    
    let stock_int: i64 = stock.parse().unwrap_or(0);
    let precio_neto_float: f64 = precio_neto.parse().unwrap_or(0.0);
    let precio_venta_float: f64 = precio_venta.parse().unwrap_or(0.0);
    let peso_float: Option<f64> = if peso.is_empty() { None } else { peso.parse().ok() };
    let marca_id_int: Option<i64> = if marca_id.is_empty() { None } else { marca_id.parse().ok() };
    
    let fecha_vencimiento_date: Option<chrono::NaiveDate> = if fecha_vencimiento.is_empty() {
        None
    } else {
        chrono::NaiveDate::parse_from_str(&fecha_vencimiento.to_string(), "%Y-%m-%d").ok()
    };

    let producto = db::Producto {
        id,
        nombre: nombre.into(),
        precio_neto: precio_neto_float,
        precio_venta: precio_venta_float,
        stock: stock_int,
        descripcion: if descripcion.is_empty() { None } else { Some(descripcion.into()) },
        peso: peso_float,
        tamano: if tamano.is_empty() { None } else { Some(tamano.into()) },
        unidad_medida: if unidad_medida.is_empty() { None } else { Some(unidad_medida.into()) },
        presentacion: if presentacion.is_empty() { None } else { Some(presentacion.into()) },
        codigo: if codigo.is_empty() { None } else { Some(codigo.into()) },
        activo,
        fecha_vencimiento: fecha_vencimiento_date,
        marca_id: marca_id_int,
    };

    let updated = db::actualizar_producto(&conn, &producto)?;
    Ok(updated)
}

/// Agrega un nuevo producto a la base de datos
pub fn add_product(
    nombre: slint::SharedString,
    precio_neto: slint::SharedString,
    precio_venta: slint::SharedString,
    stock: slint::SharedString,
    descripcion: slint::SharedString,
    peso: slint::SharedString,
    tamano: slint::SharedString,
    unidad_medida: slint::SharedString,
    presentacion: slint::SharedString,
    codigo: slint::SharedString,
    activo: bool,
    fecha_vencimiento: slint::SharedString,
    marca_id: slint::SharedString,
) -> Result<i64, Box<dyn std::error::Error>> {
    let conn = db::open_connection()?;
    
    let stock_int: i64 = stock.parse().unwrap_or(0);
    let precio_neto_float: f64 = precio_neto.parse().unwrap_or(0.0);
    let precio_venta_float: f64 = precio_venta.parse().unwrap_or(0.0);
    let peso_float: Option<f64> = if peso.is_empty() { None } else { peso.parse().ok() };
    let marca_id_int: Option<i64> = if marca_id.is_empty() { None } else { marca_id.parse().ok() };
    
    let fecha_vencimiento_date: Option<chrono::NaiveDate> = if fecha_vencimiento.is_empty() {
        None
    } else {
        chrono::NaiveDate::parse_from_str(&fecha_vencimiento.to_string(), "%Y-%m-%d").ok()
    };

    let producto_nuevo = db::ProductoNuevo {
        nombre: nombre.into(),
        precio_neto: precio_neto_float,
        precio_venta: precio_venta_float,
        stock: stock_int,
        descripcion: if descripcion.is_empty() { None } else { Some(descripcion.into()) },
        peso: peso_float,
        tamano: if tamano.is_empty() { None } else { Some(tamano.into()) },
        unidad_medida: if unidad_medida.is_empty() { None } else { Some(unidad_medida.into()) },
        presentacion: if presentacion.is_empty() { None } else { Some(presentacion.into()) },
        codigo: if codigo.is_empty() { None } else { Some(codigo.into()) },
        activo,
        fecha_vencimiento: fecha_vencimiento_date,
        marca_id: marca_id_int,
    };

    let id = db::crear_producto(&conn, &producto_nuevo)?;
    Ok(id)
}

/// Obtiene los detalles completos de un producto por ID
pub fn get_product_details(id: i64) -> Result<Option<db::ProductoConMarca>, Box<dyn std::error::Error>> {
    let conn = db::open_connection()?;
    
    let producto = db::obtener_producto_por_id(&conn, id)?;
    
    if let Some(p) = producto {
        let marca_nombre = p.marca_id.and_then(|mid| {
            db::obtener_marca_por_id(&conn, mid).ok().flatten().map(|m| m.nombre)
        });
        
        Ok(Some(db::ProductoConMarca {
            id: p.id,
            nombre: p.nombre,
            precio_neto: p.precio_neto,
            precio_venta: p.precio_venta,
            stock: p.stock,
            descripcion: p.descripcion,
            peso: p.peso,
            tamano: p.tamano,
            unidad_medida: p.unidad_medida,
            presentacion: p.presentacion,
            codigo: p.codigo,
            activo: p.activo,
            fecha_vencimiento: p.fecha_vencimiento,
            marca_id: p.marca_id,
            marca_nombre,
        }))
    } else {
        Ok(None)
    }
}

// ==================== FUNCIONES PARA MARCAS ====================

/// Obtiene todas las marcas como filas para la UI
#[allow(dead_code)]
pub fn get_brand_rows() -> Result<ModelRc<InventoryRow>, Box<dyn std::error::Error>> {
    let conn = db::open_connection()?;
    let marcas = db::obtener_marcas(&conn)?;

    let rows: Vec<InventoryRow> = marcas
        .into_iter()
        .map(|m| {
            let rif_str = m.rif.unwrap_or_else(|| "-".to_string());
            
            ModelRc::from(Rc::new(VecModel::from(vec![
                StandardListViewItem::from(slint::SharedString::from(m.id.to_string())),
                StandardListViewItem::from(slint::SharedString::from(m.nombre)),
                StandardListViewItem::from(slint::SharedString::from(rif_str)),
            ])))
        })
        .collect();

    Ok(ModelRc::from(Rc::new(VecModel::from(rows))))
}

/// Crea una nueva marca
pub fn add_brand(
    nombre: slint::SharedString,
    descripcion: slint::SharedString,
    logo: slint::SharedString,
    rif: slint::SharedString,
) -> Result<i64, Box<dyn std::error::Error>> {
    let conn = db::open_connection()?;

    let marca_nueva = db::MarcaNueva {
        nombre: nombre.into(),
        descripcion: if descripcion.is_empty() { None } else { Some(descripcion.into()) },
        logo: if logo.is_empty() { None } else { Some(logo.into()) },
        rif: if rif.is_empty() { None } else { Some(rif.into()) },
    };

    let id = db::crear_marca(&conn, &marca_nueva)?;
    Ok(id)
}
