use std::iter::Successors;
use std::net::IpAddr;

use actix_web::Responder;
use sqlx::types::ipnetwork::IpNetwork;
use sqlx::{PgPool, postgres::PgPoolOptions, Error};
use uuid::Uuid;
use chrono::Utc;
use crate::models::{
    user::{User,RefreshToken, LoginAttempt}, 
    device::{Device, DeviceResponse, DeviceEditRequest, DeviceRegisterRequest}
                };
use crate::config::Config;


#[derive(Clone)]
pub struct DeviceDatabase {
    pub pool: PgPool,
}


impl DeviceDatabase {
    pub async fn new(config: &Config) -> Result<Self, Error> {
        let pool: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
            .max_connections(config.database.max_connections)
            .connect(&config.database.url)
            .await?;

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await?;

        Ok(Self { pool })
    }

    pub async fn create_device(
        &self, userid: Uuid, nominatedid: &str, userusername: &str,  category: &str, model: &str, 
        username: &str, hostname: &str, port_no: i64, ip_addr: Option<&IpNetwork>) -> Result<Device, Error> {
        println!("<<<<< create_device in [Device Database] is being called >>>>>");

        sqlx::query_as!(Device,
            r#"
                INSERT INTO device (userid, nominatedid, userusername, category, model, username, hostname, port_no, ip_addr)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                RETURNING 
                    id AS "id!", 
                    nominatedid AS "nominatedid!",
                    userid AS "userid!",
                    userusername AS "userusername!", 
                    category AS "category!", 
                    model AS "model!", 
                    username AS "username!", 
                    hostname AS "hostname!", 
                    port_no AS "port_no!", 
                    ip_addr AS "ip_addr!", 
                    is_active AS "is_active!", 
                    created_at AS "created_at!", 
                    updated_at AS "updated_at!"
                "#,
                userid, 
                nominatedid, 
                userusername, 
                category, 
                model, 
                username, 
                hostname, 
                port_no, 
                ip_addr,
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn find_device_by_userid(&self, userId: Uuid) -> Result<Option<Device>, Error> {
        println!("<<<<< create_device in [Device Database] is being called >>>>>");

        sqlx::query_as!(
            Device, 
            r#"
            SELECT * FROM Device WHERE userid = $1
            "#,
            userId
        )
        .fetch_optional(&self.pool)
        .await
    }

}

