#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum Medida {
    Unidad = 1,
    Kilogramo = 2,
    Gramo = 3,
    Litro = 4,
    Mililitro = 5,
}

impl Medida {
    pub fn from_i32(id: i32) -> Self {
        match id {
            2 => Self::Kilogramo,
            3 => Self::Gramo,
            4 => Self::Litro,
            5 => Self::Mililitro,
            _ => Self::Unidad,
        }
    }

    pub fn info(&self) -> (&'static str, &'static str) {
        match self {
            Self::Unidad => ("Unidad", "un"),
            Self::Kilogramo => ("Kilogramo", "kg"),
            Self::Gramo => ("Gramo", "g"),
            Self::Litro => ("Litro", "lt"),
            Self::Mililitro => ("Mililitro", "ml"),
        }
    }
    pub fn todos_los_nombres() -> Vec<String> {
        vec![
            Self::Unidad.info().0.to_string(),
            Self::Kilogramo.info().0.to_string(),
            Self::Gramo.info().0.to_string(),
            Self::Litro.info().0.to_string(),
            Self::Mililitro.info().0.to_string(),
        ]
    }
}
