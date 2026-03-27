use crate::db;
use crate::models::producto::{Producto as DbProducto, ProductoNuevo};
use slint::{ModelRc, SharedString, StandardListViewItem, VecModel};
use std::rc::Rc;


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

    // Actualizar caché en productos.rs
    let product_infos: Vec<crate::productos::ProductInfo> = productos
        .iter()
        .map(|p| crate::productos::ProductInfo {
            id: p.id,
            nombre: p.nombre.clone(),
            precio_venta: p.precio_venta,
            stock: p.stock as f64,
            activo: p.activo,
        })
        .collect();

    crate::productos::update_cache_from_info(product_infos);

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

