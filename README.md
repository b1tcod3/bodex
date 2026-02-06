# Bodex - Sistema de Gestión de Inventario

Bodex es una aplicación de escritorio para la gestión integral de inventario, desarrollada en Rust con interfaz gráfica moderna.

## Características

- **Autenticación Segura**: Sistema de login con roles (Admin, Operador, Vendedor)
- **Gestión de Productos**: CRUD completo con control de stock, precios y fechas de vencimiento
- **Gestión de Marcas**: Administración de marcas asociadas a productos
- **Sistema de Ventas**: Registro de ventas con detalles y seguimiento
- **Dashboard Interactivo**: Vista general con métricas y estadísticas
- **Interfaz Moderna**: Diseño oscuro con acentos neón y animaciones fluidas

## Requisitos

- Rust 2021 Edition
- Windows 10/11
- SQLite (incluido vía rusqlite bundled)

## Instalación

```bash
# Clonar el repositorio
git clone <repo-url>
cd bodex

# Compilar en modo release
cargo build --release

# Ejecutar la aplicación
cargo run --release
```

## Credenciales por Defecto

| Usuario | Contraseña | Rol |
|---------|------------|-----|
| admin   | admin      | Admin |

## Estructura del Proyecto

```
bodex/
├── src/
│   ├── main.rs              # Punto de entrada
│   ├── models.rs            # Modelos de datos (Usuario, Producto, Marca, Venta)
│   ├── db/
│   │   ├── mod.rs           # Orquestador de base de datos
│   │   ├── usuarios.rs      # CRUD de usuarios
│   │   ├── productos.rs     # CRUD de productos
│   │   ├── marcas.rs        # CRUD de marcas
│   │   └── ventas.rs        # Gestión de ventas
│   └── ui_handlers.rs       # Callbacks de interfaz
├── ui/
│   ├── app_window.slint     # Ventana principal
│   ├── views/
│   │   ├── login_view.slint # Pantalla de login
│   │   ├── dashboard_view.slint
│   │   └── producto/
│   │       ├── lista_productos.slint
│   │       └── nuevo_producto.slint
│   └── components/          # Componentes reutilizables
├── Cargo.toml
├── config.toml
└── build.rs
```

## Base de Datos

Bodex utiliza SQLite para persistencia. El archivo `bodex.db` se crea automáticamente en el directorio de la aplicación.

**Tablas principales:**
- `usuarios`: Gestión de usuarios y autenticación
- `productos`: Catálogo de productos
- `marcas`: Catálogo de marcas
- `ventas`: Registro de ventas
- `detalles_venta`: Detalles de cada venta

## Tecnologías

| Categoría | Tecnología |
|-----------|------------|
| Lenguaje | Rust 2021 |
| UI Framework | Slint 1.3.2 |
| Base de Datos | SQLite (rusqlite) |
| Auth | bcrypt |
| Fecha/Hora | chrono |

## Licencia

MIT
