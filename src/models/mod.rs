use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTIme, utc};
use validator::validate;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct user {
     pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub full_name: String,
    pub is_active: bool,
    pub is_verified: bool,
    pub verification_token: Option<String>,
    pub reset_password_token: Option<String>,
    pub reset_password_expires: Option<DateTime<Utc>>,
    pub last_login: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)] 
pub struct RegisterRequest {
    #[validate(email)]
    pub email:String,

    #[validate(length(min = 3, max = 30))]
    pub username:String,

    #[validate(length(min = 8))]
    pub password: String,

    #[validate(length(min = 1))]
    pub full_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min=1))]  
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserResponse, 
}

#[derice(Debug, Serialize)]
pub struct UserResponse {
    pub id:id,
    pub email: String,
    pub username: String,
    pub full_name: String,
    pub is_verified: bool,
    pub created_at: DateTIme<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token:String,
}

#[derive(Debug, Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: Option<String>,
}

#[derive(debug, Deserialize, Validate)]
pub struct ForgotPasswordRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ResetPasswordRequest {
    #[validate(length(min = 8))]
    pub new_password: String,
    pub token: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    #[validate(length(min = 8))]
    pub new_password: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub revoked: bool,
}

#[derive(Debug, Serialize)]
pub struct LoginAttempt {
    pub email: String,
    pub ip_address: Option<String>,
    pub success: bool,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            message: None,
        }
    }
    
    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            message: None,
        }
    }
    
    pub fn message(message: String) -> Self {
        Self {
            success: true,
            data: None,
            error: None,
            message: Some(message),
        }
    }
}
