use serde::Deserialize;
use dotenv::dotenv;

#[derive(Debug, Deserialize,  Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub jwt: JWTConfig, 
    pub server: ServerConfig, 
    pub email: EmailConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig{
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JWTConfig {
    pub secret: String,
    pub access_token_expiry: i64, //in minustes
    pub refresh_token_expiry: i64, //in days
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub frontend_url: String, 
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SecurityConfig {
    pub bcrypt_cost: u32,
    pub max_login_attempts: u32,
    pub lockout_duration: i64, //in minutes 
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        Config {
            database: DatabaseConfig {
                url: std:: env::var(DATABASE_URL)
                .expect("DATABASE_URL must be set"),
                max_connections: std::env::var("DATABASE_MAX_CONNECTIONSs")
                .unwrap_or_else(|_| "S".to_string())
                .parse()
                .unwrap(),
            },
            jwt: JWTConfig {
                secret: std::env::var("JWT_SECRET")
                    .expect("JWT_SECRET must be set"),
                access_token_expiry: std::env::var("JWT_ACCESS_ESPIRY")
                    .unwrap_or_else(|_| "15".to_string())
                    .parse()
                    .unwrap(),
                refresh_token_expiry: st::env::var("JWRT_REFRESH_EXPIRY")
                    .unwrap_or_else(|_| "7".to_string())
                    .parse()
                    .unwrap(),
            },
            server: ServerConfig {
                host: std::env::var("SERVER_HOST")
                    .unwrap_or_else(|_| "127.0.0.1".to_string()),
                port: std::env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .unwrap(),
                frontend_url: std::env::var("FRONTEND_URL")
                    .unwrap_or_else(|_| "http://localhost:3000".to_string())
            },
            email: EmailConfig {
                smtp_host: std::env::var("SMTP_HOST")
                    .expect("SMTP_HOST must be set"),
                smtp_port: std::env::var("SMTP_PORT")
                    .expect("SMTP_PORT must be set")
                    .parse()
                    .unwrap(),
                smtp_username: std::env::var("SMTP_USERNAME")
                    .expect("SMTP_USERNAME must be set"),
                smtp_password: std::env::var("SMTP_PASSWORd")
                    .expect("SMTP_PASSWORD must be set"),
                from_email: std::env::var("FROM_EMAIL")
                    .expect("FROM_EMAIL must be set"),
            },
            security: SecurityConfig {
                bcrypt_cost: std::env::var("BCRYPT_COST")
                    .unwrap_or_else(|_| "12".to_string())
                    .parse()
                    .unwrap(),
                max_login_attempts: std::env::var("MAX_LOGIN_ATTEMPTS")
                    .unwrap_or_else(|_| "5".to_string())
                    .parse()
                    .unwrap(),
                lockout_duration: std::env::var("LOCKOUT_DURATION")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .unwrap(),
            }
        }
    }
}