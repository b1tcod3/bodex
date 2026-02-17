#[derive(Debug, Clone, PartialEq)]
pub enum Rol {
    Admin,
    Operador,
    Vendedor,
}

impl From<String> for Rol {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Admin" => Rol::Admin,
            "Operador" => Rol::Operador,
            _ => Rol::Vendedor,
        }
    }
}

impl ToString for Rol {
    fn to_string(&self) -> String {
        match self {
            Rol::Admin => "Admin".to_string(),
            Rol::Operador => "Operador".to_string(),
            Rol::Vendedor => "Vendedor".to_string(),
        }
    }
}
