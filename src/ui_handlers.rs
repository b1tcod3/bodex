use crate::inventory;
use crate::db;
use slint::{ComponentHandle, Weak, SharedString, ModelRc, VecModel, StandardListViewItem};
use crate::AppWindow;
use std::rc::Rc;

/// Configura todos los callbacks de la UI
pub fn setup_callbacks(ui: &AppWindow) {
    let ui_handle = ui.as_weak();

    // ==========================================
    // 1. MANEJO DE SESIÓN (LOGIN / LOGOUT)
    // ==========================================

    ui.on_attempt_login({
        let ui_handle = ui_handle.clone();
        move |user, pass| {
            if let Some(ui) = ui_handle.upgrade() {
                match db::open_connection() {
                    Ok(conn) => {
                        // Llamada al nuevo submódulo de usuarios
                        match db::usuarios::validar_usuario(&conn, user.as_str(), pass.as_str()) {
                            Ok(Some(usuario)) => {
                                println!("Acceso concedido: {} - Rol: {:?}", usuario.username, usuario.rol);
                                
                                // Cambiar a la vista principal
                                ui.set_current_view("dashboard".into());
                                
                                // Cargar inventario inmediatamente después de loguear
                                refresh_ui(ui_handle.clone());
                            }
                            Ok(None) => {
                                eprintln!("Credenciales incorrectas para el usuario: {}", user);
                                // Opcional: ui.set_error_message("Usuario o clave incorrectos".into());
                            }
                            Err(e) => eprintln!("Error de base de datos en login: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Error conectando a la DB: {}", e),
                }
            }
        }
    });

    ui.on_logout({
        let ui_handle = ui_handle.clone();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                println!("Sesión finalizada.");
                ui.set_current_view("login".into());
            }
        }
    });

    // ==========================================
    // 2. GESTIÓN DE PRODUCTOS (INVENTARIO)
    // ==========================================

    ui.on_add_product({
        let refresh = create_refresh_handle(ui_handle.clone());
        move |nombre, p_neto, p_venta, stock, desc, peso, tam, u_medida, pres, cod, act, f_venc, m_id| {
            match inventory::add_product(
                nombre, p_neto, p_venta, stock, desc, 
                peso, tam, u_medida, pres, cod, act, f_venc, m_id
            ) {
                Ok(id) => {
                    println!("Producto guardado exitosamente. ID: {}", id);
                    refresh();
                }
                Err(e) => eprintln!("Error al añadir producto: {}", e),
            }
        }
    });

    ui.on_delete_product({
        let refresh = create_refresh_handle(ui_handle.clone());
        move |index| {
            match inventory::delete_product_by_index(index) {
                Ok(true) => {
                    println!("Producto eliminado de la base de datos.");
                    refresh();
                }
                Ok(false) => eprintln!("No se pudo encontrar el producto seleccionado."),
                Err(e) => eprintln!("Error en eliminación: {}", e),
            }
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

    // ==========================================
    // 3. UTILIDADES Y CONTROL DE APP
    // ==========================================

    ui.on_refresh_inventory({
        let refresh = create_refresh_handle(ui_handle.clone());
        move || refresh()
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

/// Carga inicial: No cargamos datos hasta que el usuario se autentique
pub fn load_initial_data(_ui: &AppWindow) {
    println!("Sistema Bodex iniciado. Esperando login...");
}

/// Helper para crear un closure de refresco reutilizable
fn create_refresh_handle(ui_handle: Weak<AppWindow>) -> impl Fn() {
    move || refresh_ui(ui_handle.clone())
}

/// Sincroniza la tabla de Slint con la base de datos actual
fn refresh_ui(ui_handle: Weak<AppWindow>) {
    match inventory::get_inventory_rows() {
        Ok(rows) => {
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_inventory_rows(rows);
            }
        }
        Err(e) => eprintln!("Error al refrescar la interfaz: {}", e),
    }
}
