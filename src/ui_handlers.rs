use crate::db;
use crate::inventory;
use crate::AppWindow;
use slint::{ComponentHandle, SharedString, Weak};

/// Filtra caracteres no numéricos de un string
/// Permite dígitos, punto decimal (opcional) y signo negativo (opcional)
fn filter_numeric(input: &str, allow_decimal: bool, allow_negative: bool) -> String {
    let mut result = String::new();
    let mut has_dot = false;
    let mut has_digit = false;
    
    for char in input.chars() {
        // Dígito: siempre permitir
        if char.is_ascii_digit() {
            result.push(char);
            has_digit = true;
        }
        // Punto decimal: permitir solo uno si allow_decimal es true
        else if char == '.' && allow_decimal && !has_dot {
            result.push(char);
            has_dot = true;
        }
        // Signo negativo: permitir solo al inicio si allow_negative es true
        else if char == '-' && allow_negative && result.is_empty() {
            result.push(char);
        }
    }
    
    // Si no hay dígitos, retornar vacío
    if !has_digit {
        return String::new();
    }
    
    result
}

pub fn setup_callbacks(ui: &AppWindow) {
    let ui_handle = ui.as_weak();

    // === CALLBACK PARA VALIDACIÓN NUMÉRICA EN TIEMPO REAL ===
    // Este callback es usado por InputNumber para filtrar caracteres no numéricos
    // Parámetros: (texto, permitir_decimal, permitir_negativo)
    // Retorna el texto filtrado con solo caracteres numéricos válidos
    ui.on_validate_numeric({
        move |input: SharedString, allow_decimal: bool, allow_negative: bool| {
            let filtered = filter_numeric(input.as_str(), allow_decimal, allow_negative);
            SharedString::from(filtered)
        }
    });

    // === CALLBACK PARA VERIFICAR SKU ===
    ui.on_verificar_sku({
        let ui_handle = ui_handle.clone();
        move |sku: SharedString| {
            if let Some(ui) = ui_handle.upgrade() {
                // Si el SKU está vacío, limpiar todos los errores
                if sku.trim().is_empty() {
                    ui.set_sku_duplicado(false);
                    ui.set_mensaje_error("".into());
                    return;
                }
                
                // 1. Primero validar el formato del SKU (caracteres)
                let validacion_formato = inventory::validar_formato_sku(sku.as_str());
                if !validacion_formato.es_valido {
                    // Error de formato: limpiar error de duplicado, mostrar error de formato
                    ui.set_sku_duplicado(false);
                    if let Some(error_msg) = validacion_formato.error {
                        ui.set_mensaje_error(error_msg.into());
                    } else {
                        ui.set_mensaje_error("El SKU no tiene el formato válido".into());
                    }
                    return;
                }
                
                // 2. El formato es válido, ahora verificar unicidad en la base de datos
                match db::open_connection() {
                    Ok(conn) => {
                        match db::productos::existe_sku(&conn, sku.as_str()) {
                            Ok(existe) => {
                                if existe {
                                    // SKU duplicado en BD
                                    ui.set_sku_duplicado(true);
                                    ui.set_mensaje_error("El código SKU ya existe en la base de datos".into());
                                } else {
                                    // Todo bien: limpiar todos los errores
                                    ui.set_sku_duplicado(false);
                                    ui.set_mensaje_error("".into());
                                }
                            }
                            Err(e) => {
                                eprintln!("Error al verificar SKU: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error de conexión: {}", e);
                    }
                }
            }
        }
    });

    // === CALLBACK PARA MANEJAR CAMBIO DE TEXTO EN SKU ===
    // Este callback se llama desde InputText cuando el usuario escribe
    // Parámetros: (texto_actual)
    // Limpia los errores de SKU duplicado y mensaje general
    ui.on_changed({
        let ui_handle = ui_handle.clone();
        move |texto: SharedString| {
            if let Some(ui) = ui_handle.upgrade() {
                // Limpiar errores cuando el usuario empieza a escribir
                // IMPORTANTE: No limpiar 'error-sku' aquí, ya que eso es manejado por Slint
                // Limpiar solo el estado de duplicado y el mensaje general
                ui.set_sku_duplicado(false);
                ui.set_mensaje_error("".into());
            }
        }
    });

    // 1. LOGIN
    ui.on_attempt_login({
        let ui_handle = ui_handle.clone();
        move |user: SharedString, pass: SharedString| {
            if let Some(ui) = ui_handle.upgrade() {
                match db::open_connection() {
                    Ok(conn) => {
                        match db::usuarios::validar_usuario(&conn, user.as_str(), pass.as_str()) {
                            Ok(Some(_usuario)) => {
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
    // Callback desde Slint: (nombre, p_neto, p_venta, stock, desc, peso, tam, u_med, pres, cod, venc, activo, m_id, cat_id, subcat_id)
    // Función inventory::add_product: (nombre, precio_neto, precio_venta, stock, descripcion, codigo, activo_str, marca_id,
    //                                   medida_p_id, cantidad_p, medida_s_id, cantidad_s, empaque_id, categoria_id, subcategoria_id)
    ui.on_add_product({
        let ui_handle = ui_handle.clone();
        move |nombre,
              p_neto,
              p_venta,
              stock,
              desc,
              peso,      // -> medida_p_id
              tam,       // -> cantidad_p
              u_medida,  // -> medida_s_id
              pres,      // -> cantidad_s
              cod,
              _f_venc,   // No usado actualmente
              activo,    // Es SharedString desde Slint (string)
              m_id,
              cat_id,    // -> categoria_id (nuevo parámetro 14)
              subcat_id| // -> subcategoria_id (nuevo parámetro 15)
            
            {
            
            // === VALIDACIÓN PREVIA EN RUST ===
            // Validación completa de SKU (formato + unicidad) + Validación financiera
            {
                if let Some(ui) = ui_handle.upgrade() {
                    // Validar formato y unicidad del SKU
                    if !cod.trim().is_empty() {
                        let validacion_sku = inventory::validar_sku_completo(cod.as_str());
                        if !validacion_sku.es_valido {
                            ui.set_sku_duplicado(true);
                            if let Some(error_msg) = validacion_sku.error {
                                ui.set_mensaje_error(error_msg.into());
                            }
                            return; // No continuar con el guardado
                        }
                    }
                    
                    // Validar campos requeridos
                    if nombre.trim().is_empty() {
                        ui.set_mensaje_error("El nombre del producto es requerido".into());
                        return;
                    }
                    
                    // === VALIDACIÓN FINANCIERA ===
                    // Parseo seguro de valores numéricos
                    let neto: f64 = p_neto.as_str().trim().parse().unwrap_or(-1.0);
                    let venta: f64 = p_venta.as_str().trim().parse().unwrap_or(-1.0);
                    
                    // Validar que los precios sean positivos
                    if neto <= 0.0 {
                        ui.set_mensaje_error("El costo neto debe ser un número positivo mayor a 0".into());
                        return;
                    }
                    
                    if venta <= 0.0 {
                        ui.set_mensaje_error("El precio de venta debe ser un número positivo mayor a 0".into());
                        return;
                    }
                    
                    // Validar que no haya pérdida (precio de venta >= costo)
                    if venta < neto {
                        ui.set_mensaje_error(
                            format!(
                                "¡Pérdida detectada! El precio de venta (${:.2}) es menor al costo (${:.2})",
                                venta, neto
                            ).into()
                        );
                        return;
                    }
                    
                    // Advertencia si el margen es muy bajo (menos del 10%)
                    let margen_porcentaje = ((venta - neto) / neto) * 100.0;
                    if margen_porcentaje < 10.0 && margen_porcentaje > 0.0 {
                        println!("⚠ Advertencia: Margen de ganancia bajo ({:.1}%)", margen_porcentaje);
                        // No bloqueamos el guardado, solo mostramos advertencia en consola
                    }
                    
                    // Limpiar errores si todo está bien
                    ui.set_sku_duplicado(false);
                    ui.set_mensaje_error("".into());
                    ui.set_procesando(true);
                }
            }
            
            // IMPORTANTE: Convertir SharedString a String ANTES de tokio::spawn
            // SharedString no es Send, pero String sí lo es
            let nombre = nombre.to_string();
            let p_neto = p_neto.to_string();
            let p_venta = p_venta.to_string();
            let stock = stock.to_string();
            let desc = desc.to_string();
            let peso = peso.to_string();
            let tam = tam.to_string();
            let u_medida = u_medida.to_string();
            let pres = pres.to_string();
            let cod = cod.to_string();
            let activo_str = activo.to_string();  // Ya viene como string desde Slint
            let m_id = m_id.to_string();
            let cat_id = cat_id.to_string();
            let subcat_id = subcat_id.to_string();

            // Disparamos la tarea en un hilo de Tokio
            tokio::spawn({
                let ui_handle = ui_handle.clone();
                async move {
                    // --- HILO SECUNDARIO ---
                    // 1. Intentamos guardar el producto (operación pesada de DB)
                    let save_result = inventory::add_product(
                        SharedString::from(&nombre),
                        SharedString::from(&p_neto),
                        SharedString::from(&p_venta),
                        SharedString::from(&stock),
                        SharedString::from(&desc),
                        SharedString::from(&cod),
                        SharedString::from(&activo_str),
                        SharedString::from(&m_id),
                        // Mapeo de campos adicionales
                        SharedString::from(&peso),    // medida_p_id
                        SharedString::from(&tam),     // cantidad_p
                        SharedString::from(&u_medida), // medida_s_id
                        SharedString::from(&pres),    // cantidad_s
                        SharedString::from("1"),      // empaque_id (default)
                        // Categoría y subcategoría
                        SharedString::from(&cat_id),    // categoria_id
                        SharedString::from(&subcat_id), // subcategoria_id
                    );

                    // 2. Procesar resultado y preparar mensaje ANTES de upgrade_in_event_loop
                    let (success, error_msg) = match save_result {
                        Ok(_) => {
                            println!("Producto guardado exitosamente.");
                            (true, String::new())
                        }
                        Err(e) => {
                            let error_str = e.to_string();
                            eprintln!("Error al guardar el producto: {}", error_str);
                            
                            // Detectar tipo de error para mensaje más amigable
                            let msg = if error_str.contains("UNIQUE constraint") || error_str.contains("codigo") {
                                "El código SKU ya existe en la base de datos".to_string()
                            } else if error_str.contains("FOREIGN KEY") {
                                "Error de referencia: la marca o categoría no existe".to_string()
                            } else if error_str.contains("NOT NULL") {
                                "Faltan campos requeridos".to_string()
                            } else {
                                format!("Error al guardar: {}", error_str)
                            };
                            (false, msg)
                        }
                    };

                    // 3. Volvemos al hilo de la UI para desactivar spinner y mostrar resultado
                    if let Err(e) = ui_handle.upgrade_in_event_loop(move |ui| {
                        // Desactivar spinner
                        ui.set_procesando(false);
                        
                        if success {
                            // Éxito: Limpiar errores y navegar
                            ui.set_mensaje_error("".into());
                            ui.set_sku_duplicado(false);
                            // Refrescar datos
                            refresh_ui_from_main(&ui);
                            // Navegar a la lista de productos
                            ui.set_product_screen("lista".into());
                        } else {
                            // Error: Mostrar mensaje al usuario
                            // Verificar si es error de SKU ANTES de mover el valor
                            let es_error_sku = error_msg.contains("SKU") || error_msg.contains("código");
                            ui.set_mensaje_error(error_msg.into());
                            if es_error_sku {
                                ui.set_sku_duplicado(true);
                            }
                        }
                    }) {
                        eprintln!("Error al actualizar UI: {}", e);
                    }
                }
            });
        }
    });

    ui.on_delete_product({
        let ui_handle = ui_handle.clone();
        move |index| {
            tokio::spawn({
                let ui_handle = ui_handle.clone();
                async move {
                    let result = inventory::delete_product_by_index(index);
                    
                    // Procesar resultado ANTES de upgrade_in_event_loop
                    let success = result.is_ok();
                    if let Err(e) = &result {
                        eprintln!("Error al eliminar: {}", e);
                    } else {
                        println!("Producto eliminado exitosamente.");
                    }
                    
                    if let Err(e) = ui_handle.upgrade_in_event_loop(move |ui| {
                        if success {
                            refresh_ui_from_main(&ui);
                        }
                    }) {
                        eprintln!("Error al actualizar UI: {}", e);
                    }
                }
            });
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

/// Refresca la UI cargando datos en segundo plano (versión con Weak<AppWindow>)
fn refresh_ui(ui_handle: Weak<AppWindow>) {
    tokio::spawn(async move {
        // --- HILO SECUNDARIO ---
        // 1. Buscamos los datos crudos (operación pesada de DB)
        // Usamos get_inventory_rows_raw() que devuelve tipos Send-safe
        let filas_res = inventory::get_inventory_rows_raw();

        if let Ok(filas) = filas_res {
            // 2. Volvemos al hilo de la UI para actualizar la tabla
            // Los datos crudos (Vec<ProductRowData>) son Send
            if let Err(e) = ui_handle.upgrade_in_event_loop(move |ui| {
                // Convertir datos crudos a ModelRc DENTRO del hilo de UI
                let model_rows = inventory::raw_to_model_rows(filas);
                ui.set_inventory_rows(model_rows);
                println!("Tabla actualizada en segundo plano.");
            }) {
                eprintln!("Error al actualizar UI: {}", e);
            }
        }
    });
}

/// Refresca la UI desde el hilo principal (recibe referencia directa)
fn refresh_ui_from_main(ui: &AppWindow) {
    let ui_handle = ui.as_weak();
    
    tokio::spawn(async move {
        let filas_res = inventory::get_inventory_rows_raw();

        if let Ok(filas) = filas_res {
            if let Err(e) = ui_handle.upgrade_in_event_loop(move |ui| {
                let model_rows = inventory::raw_to_model_rows(filas);
                ui.set_inventory_rows(model_rows);
                println!("Tabla actualizada en segundo plano.");
            }) {
                eprintln!("Error al actualizar UI: {}", e);
            }
        }
    });
}

/// Carga los datos iniciales de forma asíncrona
pub fn load_initial_data(ui: &AppWindow) {
    let ui_handle = ui.as_weak();
    
    tokio::spawn(async move {
        // --- HILO SECUNDARIO ---
        // 1. Buscamos los datos crudos (operación pesada de DB)
        let filas_res = inventory::get_inventory_rows_raw();

        if let Ok(filas) = filas_res {
            // 2. Volvemos al hilo de la UI para actualizar la tabla
            if let Err(e) = ui_handle.upgrade_in_event_loop(move |ui| {
                let model_rows = inventory::raw_to_model_rows(filas);
                ui.set_inventory_rows(model_rows);
                println!("Datos iniciales cargados en segundo plano.");
            }) {
                eprintln!("Error al cargar datos iniciales: {}", e);
            }
        }
    });
}