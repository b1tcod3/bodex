use crate::models::Rol;
use chrono::NaiveDateTime; // Importamos el re-export que hicimos en mod.rs

#[derive(Debug, Clone)]
pub struct Usuario {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub rol: Rol,
    pub activo: bool,
    pub ultimo_login: Option<NaiveDateTime>,
}
