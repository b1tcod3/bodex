// 1. Declaración de los submódulos especializados
// "pub" permite que otros archivos (como ui_handlers) accedan a ellos
pub mod marcas;
pub mod productos;
pub mod usuarios;
pub mod ventas;

use rusqlite::{Connection, Result};

/// Abre o crea la conexión con el archivo de base de datos SQLite
pub fn open_connection() -> Result<Connection> {
    Connection::open("bodex.db")
}

/// Orquestador de la inicialización.
/// Llama a la creación de tablas de cada módulo en el orden correcto
/// para respetar las claves foráneas (Foreign Keys).
pub fn init_db(conn: &Connection) -> Result<()> {
    // Habilitar soporte para claves foráneas en esta conexión
    conn.execute("PRAGMA foreign_keys = ON", [])?;

    // Orden de creación respetando dependencias:
    // 1. Usuarios (Independiente)
    usuarios::create_table(conn)?;

    // 2. Marcas (Independiente)
    marcas::create_table(conn)?;

    // 2.1 Insertar marcas iniciales (Seeder)
    marcas::seed_marcas(conn)?;

    // 3. Productos (Depende de Marcas)
    productos::create_table(conn)?;

    // 4. Ventas (Independiente)
    ventas::create_table(conn)?;

    // 5. Detalles de Venta (Depende de Ventas y Productos)
    ventas::create_detalle_table(conn)?;

    // Insertar datos iniciales de configuración (Seeders)
    usuarios::seed_admin(conn)?;

    println!("Base de datos inicializada correctamente.");
    Ok(())
}
