pub mod marca;
pub mod medida;
pub mod producto;
pub mod rol;
pub mod tipo_empaque;
pub mod usuario;
pub mod venta;

// Re-exportaciones para mayor comodidad
pub use marca::{Marca, MarcaNueva};
pub use medida::Medida;
pub use producto::{Producto, ProductoConMarca, ProductoNuevo};
pub use rol::Rol;
pub use tipo_empaque::TipoEmpaque;
pub use usuario::Usuario;
pub use venta::{DetalleVenta, DetalleVentaConProducto, Venta, VentaConDetalles};
