use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct NewUserDto {
    pub id: String,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub full_name: String,
    pub is_verified: bool,
}