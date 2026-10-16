use std::iter::Successors;
use std::net::IpAddr;

use actix_web::Responder;
use sqlx::types::ipnetwork::IpNetwork;
use sqlx::{PgPool, postgres::PgPoolOptions, Error};
use uuid::Uuid;
use chrono::Utc;
use crate::models::{user::User, user::RefreshToken, user::LoginAttempt};
use crate::config::Config;


#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new(config: &Config) -> Result<Self, Error> {
        let pool = PgPoolOptions::new()
            .max_connections(config.database.max_connections)
            .connect(&config.database.url)
            .await?;

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await?;

        Ok(Self { pool })
    }

    pub async fn create_user(&self, email: &str, username: &str, password_hash: &str, full_name: &str, ) -> Result<User, Error> {
        println!("<<<<< create user in database is being called.... >>>>>");
        
          sqlx::query_as!(User,
            r#"
            INSERT INTO users (email, username, password_hash, full_name, verification_token)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, email, username, password_hash, full_name, is_active, is_verified, verification_token, reset_password_token, reset_password_expires, last_login, created_at, updated_at
            "#,
            email,
            username,
            password_hash,
            full_name,
            Some(Uuid::new_v4().to_string())
        )
        .fetch_one(&self.pool)
        .await
    }  

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, Error>{
        sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn get_user_by_id(&self, id:Uuid) -> Result<Option<User>, Error> {
        sqlx::query_as!(
            User, r#"
            SELECT * FROM Users WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn record_login_attempt (&self, email: &str, ip_address: Option<&IpNetwork>, success: bool) -> Result<(), Error> {
        sqlx::query!(
            r#"
            INSERT INTO login_attempts (email, ip_address, success) 
            VALUES ($1, $2, $3)
            "#,
            email, 
            ip_address,
            success
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }


    pub async fn update_lastLogin(&self, userId: Uuid) -> Result<(), Error> {
        sqlx::query!(
            r#"
            UPDATE users SET last_login = NOW(), updated_at = NOW()
            WHERE id = $1
            "#,
            userId
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

}