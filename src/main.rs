// 1. Declaración de módulos internos
mod db;
mod inventory;
mod models;
mod ui_handlers;

// 2. Importaciones de Slint y estándares
slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 3. Inicialización de la persistencia
    // Abrimos la conexión y nos aseguramos de que las tablas (y el admin) existan
    let conn = db::open_connection()?;
    db::init_db(&conn)?;

    // Cerramos la conexión inicial para dejar que cada handler
    // abra la suya propia si usas un enfoque de conexión por hilo,
    // o puedes mantenerla abierta si prefieres pasarla.
    drop(conn);

    // 4. Creación de la instancia de la interfaz
    let ui = AppWindow::new()?;

    // Habilitar maximizar y redimensionar
    ui.window().set_maximized(false);
    ui.window().set_minimized(false);

    // 5. Configuración de la lógica (Handlers)
    // Pasamos la referencia de la UI a nuestro módulo de callbacks
    ui_handlers::setup_callbacks(&ui);

    // 6. Carga de estado inicial (si fuera necesario)
    ui_handlers::load_initial_data(&ui);

    // 7. Ejecución del bucle principal de la aplicación
    println!("Bodex v1.0 - Sistema iniciado correctamente.");
    ui.run()?;

    Ok(())
}
