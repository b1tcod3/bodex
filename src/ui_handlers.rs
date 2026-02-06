use crate::db;
use crate::inventory;
use crate::AppWindow;
use slint::{ComponentHandle, ModelRc, SharedString, StandardListViewItem, VecModel, Weak};

pub fn setup_callbacks(ui: &AppWindow) {
    let ui_handle = ui.as_weak();

    // 1. LOGIN (Corregido con anotaciones de tipo)
    ui.on_attempt_login({
        let ui_handle = ui_handle.clone();
        move |user: SharedString, pass: SharedString| {
            // <--- Tipos explícitos añadidos
            if let Some(ui) = ui_handle.upgrade() {
                match db::open_connection() {
                    Ok(conn) => {
                        match db::usuarios::validar_usuario(&conn, user.as_str(), pass.as_str()) {
                            Ok(Some(usuario)) => {
                                ui.set_current_view("dashboard".into());
                                refresh_ui(ui_handle.clone());
                            }
                            Ok(None) => eprintln!("Credenciales incorrectas"),
                            Err(e) => eprintln!("Error DB: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Error conexión: {}", e),
                }
            }
        }
    });

    ui.on_logout({
        let ui_handle = ui_handle.clone();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_current_view("login".into());
            }
        }
    });

    // 2. GESTIÓN DE PRODUCTOS
    ui.on_add_product({
        let ui_handle = ui_handle.clone();
        move |nombre,
              p_neto,
              p_venta,
              stock,
              desc,
              peso,
              tam,
              u_medida,
              pres,
              cod,
              f_venc,
              activo,
              m_id| {
            match inventory::add_product(
                nombre, p_neto, p_venta, stock, desc, peso, tam, u_medida, pres, cod, f_venc,
                activo, m_id,
            ) {
                Ok(_) => refresh_ui(ui_handle.clone()),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    });

    ui.on_delete_product({
        let ui_handle = ui_handle.clone();
        move |index| match inventory::delete_product_by_index(index) {
            Ok(_) => refresh_ui(ui_handle.clone()),
            Err(e) => eprintln!("Error: {}", e),
        }
    });

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

    ui.on_refresh_inventory({
        let ui_handle = ui_handle.clone();
        move || refresh_ui(ui_handle.clone())
    });

    ui.on_close_app({
        let ui_handle = ui_handle.clone();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                let _ = ui.hide();
            }
        }
    });
}

fn refresh_ui(ui_handle: Weak<AppWindow>) {
    match inventory::get_inventory_rows() {
        Ok(rows) => {
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_inventory_rows(rows);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub fn load_initial_data(_ui: &AppWindow) {}
