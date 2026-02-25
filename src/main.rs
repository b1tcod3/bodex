// 1. Declaración de módulos internos
mod db;
mod inventory;
mod models;
mod ui_handlers;

// 2. Importaciones de Slint y estándares
slint::include_modules!();

// 3. Cambiamos main a una función asíncrona de Tokio
#[tokio::main]
async fn main() {
    // --- CONFIGURACIÓN DEL RENDERIZADOR ---
    // Forzamos el backend de software antes de que Slint se inicialice.
    // Esto evita errores de GPU en entornos como WSL, VMs o drivers antiguos.
    #[cfg(feature = "desktop")]
    {
        std::env::set_var("SLINT_BACKEND", "software");
    }

    // Verificar si hay display disponible (evita pánicos en servidores puros)
    let has_display = std::env::var("DISPLAY").is_ok()
        || cfg!(target_os = "windows")
        || cfg!(target_os = "macos");

    // 3. Inicialización de la persistencia (Base de Datos)
    let conn = match db::open_connection() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error crítico: No se pudo abrir la base de datos: {}", e);
            return;
        }
    };

    if let Err(e) = db::init_db(&conn) {
        eprintln!("Error crítico: No se pudo inicializar las tablas: {}", e);
        return;
    }
    // Cerramos la conexión inicial; los handlers abrirán las suyas propias
    drop(conn);

    if has_display {
        // 4. Creación de la instancia de la interfaz
        let ui = match AppWindow::new() {
            Ok(ui) => ui,
            Err(e) => {
                eprintln!("Error al crear la ventana principal: {}", e);
                return;
            }
        };

        // Configuración de ventana
        ui.window().set_maximized(false);

        // 5. Configuración de la lógica (Handlers de botones y eventos)
        ui_handlers::setup_callbacks(&ui);

        // 6. Carga de estado inicial (Cargar productos en la tabla, etc.)
        ui_handlers::load_initial_data(&ui);

        // 7. Ejemplo de carga de datos en un hilo secundario con Tokio
        let ui_handle = ui.as_weak();
        tokio::spawn(async move {
            // Simulamos una carga pesada o consulta a DB
            println!("Hilo secundario: Cargando datos...");
            
            // Aquí llamarías a tu lógica de base de datos
            // let resultados = inventory::get_inventory_rows();

            // Para actualizar la UI, debemos "regresar" al hilo principal
            ui_handle.upgrade_in_event_loop(|ui| {
                // ui_handlers::load_initial_data(&ui);
                println!("UI actualizada desde Tokio");
            }).unwrap();
        });

        // 8. Ejecución del bucle principal
        println!("-----------------------------------------");
        println!("BODEX v1.0 - Gestión de Inventario");
        println!("Estado: Iniciado con Renderizador de Software");
        println!("-----------------------------------------");

        if let Err(e) = ui.run() {
            eprintln!("Error durante la ejecución de la App: {}", e);
        }
    } else {
        // Modo sin GUI (Headless)
        println!("Bodex v1.0 - Modo sin GUI detectado.");
        println!("La base de datos se verificó correctamente.");
        println!(
            "Nota: Para ver la interfaz, asegúrese de tener un servidor X11 o Wayland activo."
        );
    }
}
