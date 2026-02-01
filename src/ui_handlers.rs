use crate::inventory;
use slint::{ComponentHandle, Weak};
use crate::AppWindow;

/// Configura todos los callbacks de la UI
pub fn setup_callbacks(ui: &AppWindow) {
    let ui_handle = ui.as_weak();

    // Callback para agregar producto
    ui.on_add_product({
        let refresh = create_refresh_handle(ui_handle.clone());
        move |nombre, precio_neto, precio_venta, stock, descripcion, peso, tamano, unidad_medida, presentacion, codigo, activo, fecha_vencimiento, marca_id| {
            match inventory::add_product(
                nombre, 
                precio_neto, 
                precio_venta, 
                stock, 
                descripcion, 
                peso, 
                tamano, 
                unidad_medida, 
                presentacion, 
                codigo, 
                activo, 
                fecha_vencimiento, 
                marca_id
            ) {
                Ok(id) => {
                    println!("Producto creado con ID: {}", id);
                    refresh();
                }
                Err(e) => {
                    eprintln!("Error al crear producto: {}", e);
                }
            }
        }
    });

    // Callback para eliminar producto
    ui.on_delete_product({
        let refresh = create_refresh_handle(ui_handle.clone());
        move |index| {
            match inventory::delete_product_by_index(index) {
                Ok(deleted) => {
                    if deleted {
                        println!("Producto eliminado correctamente");
                        refresh();
                    } else {
                        eprintln!("No se pudo eliminar el producto");
                    }
                }
                Err(e) => {
                    eprintln!("Error al eliminar producto: {}", e);
                }
            }
        }
    });

    // Callback para obtener producto para editar
    ui.on_get_product_for_edit({
        let ui_handle = ui_handle.clone();
        move |index| {
            if let Some(product) = inventory::get_product_by_index(index) {
                if let Some(ui) = ui_handle.upgrade() {
                    ui.set_edit_product_id(product.id as i32);
                    ui.set_edit_product_name(product.nombre.into());
                    ui.set_edit_product_precio_venta(product.precio_venta.to_string().into());
                    ui.set_edit_product_stock(product.stock.to_string().into());
                }
            }
        }
    });

    // Callback para actualizar producto
    ui.on_update_product({
        let refresh = create_refresh_handle(ui_handle.clone());
        move |id, nombre, precio_neto, precio_venta, stock, descripcion, peso, tamano, unidad_medida, presentacion, codigo, activo, fecha_vencimiento, marca_id| {
            match inventory::update_product(
                id as i64, 
                nombre, 
                precio_neto, 
                precio_venta, 
                stock, 
                descripcion, 
                peso, 
                tamano, 
                unidad_medida, 
                presentacion, 
                codigo, 
                activo, 
                fecha_vencimiento, 
                marca_id
            ) {
                Ok(updated) => {
                    if updated {
                        println!("Producto actualizado correctamente");
                        refresh();
                    } else {
                        eprintln!("No se pudo actualizar el producto");
                    }
                }
                Err(e) => {
                    eprintln!("Error al actualizar producto: {}", e);
                }
            }
        }
    });

    // Callback para refrescar inventario manualmente
    ui.on_refresh_inventory({
        let refresh = create_refresh_handle(ui_handle.clone());
        move || {
            refresh();
        }
    });

    // Callback para cerrar la aplicación
    ui.on_close_app({
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                ui.hide().unwrap();
            }
        }
    });
}

/// Carga los datos iniciales del inventario en la UI
pub fn load_initial_data(ui: &AppWindow) {
    refresh_ui(ui.as_weak());
}

/// Crea un closure que refresca la UI
fn create_refresh_handle(ui_handle: Weak<AppWindow>) -> impl Fn() {
    move || refresh_ui(ui_handle.clone())
}

/// Refresca la tabla de inventario en la UI
fn refresh_ui(ui_handle: Weak<AppWindow>) {
    match inventory::get_inventory_rows() {
        Ok(rows) => {
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_inventory_rows(rows);
            }
        }
        Err(e) => {
            eprintln!("Error al cargar inventario: {}", e);
        }
    }
}
