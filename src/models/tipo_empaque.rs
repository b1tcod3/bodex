#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum TipoEmpaque {
    Individual = 1,
    Caja = 2,
    Bolsa = 3,
    Frasco = 4,
    Botella = 5,
    Lata = 6,
    Docena = 7,
    SixPack = 8,
}

impl TipoEmpaque {
    pub fn from_i32(id: i32) -> Self {
        match id {
            2 => Self::Caja,
            3 => Self::Bolsa,
            4 => Self::Frasco,
            5 => Self::Botella,
            6 => Self::Lata,
            7 => Self::Docena,
            8 => Self::SixPack,
            _ => Self::Individual,
        }
    }

    pub fn info(&self) -> (&'static str, &'static str) {
        match self {
            Self::Individual => ("Individual", "ind"),
            Self::Caja => ("Caja", "cj"),
            Self::Bolsa => ("Bolsa", "bls"),
            Self::Frasco => ("Frasco", "fr"),
            Self::Botella => ("Botella", "bt"),
            Self::Lata => ("Lata", "lt"),
            Self::Docena => ("Docena", "dz"),
            Self::SixPack => ("SixPack", "6pk"),
        }
    }
    pub fn todos_tipos() -> Vec<Self> {
        vec![
            Self::Individual,
            Self::Caja,
            Self::Bolsa,
            Self::Frasco,
            Self::Botella,
            Self::Lata,
            Self::Docena,
            Self::SixPack,
        ]
    }
}
