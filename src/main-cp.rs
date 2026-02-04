slint::include_modules!();

mod db;
mod inventory;
mod ui_handlers;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Inicializar Base de Datos
    let conn = db::open_connection()?;
    db::init_db(&conn)?;
    drop(conn);

    // 2. Cargar Ventana
    let ui = AppWindow::new()?;

    // 3. Configurar callbacks de la UI
    ui_handlers::setup_callbacks(&ui);

    // 4. Cargar datos iniciales
    ui_handlers::load_initial_data(&ui);

    // 5. Ejecutar aplicación
    ui.run()?;
    Ok(())
}
