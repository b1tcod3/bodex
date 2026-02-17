#[derive(Debug, Clone)]
pub struct Marca {
    pub id: i64,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub logo: Option<String>,
    pub rif: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MarcaNueva {
    pub nombre: String,
    pub descripcion: Option<String>,
    pub logo: Option<String>,
    pub rif: Option<String>,
}

