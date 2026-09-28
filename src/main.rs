mod auth;
mod config;
mod db;
mod handlers;
mod models;
mod dto;

use actix_web::{App, HttpResponse, HttpServer, Responder, web::{self, Json}, middleware::Logger };
use actix_cors::Cors;
use actix_web_httpauth::middleware::HttpAuthentication;

use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use dotenv::dotenv;
use std::{env, fmt::format};


use crate::{
    auth::{AuthService},
    config::Config,
    db::Database,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("infor"));

    dotenv().ok();

    let config = Config::from_env();
    let config_data = web::Data::new(config.clone());

    println!("Trying to connect Database");
    let databaseconnection = match Database::new(&config).await {
        Ok(db) => {
            println!("Database Connected Successfully...");
            db
        }
        Err(e) => {
            eprintln!("Failed to connect Database : {}", e);
            std::process::exit(1);
        }
    };

    let db_data = web::Data::new(databaseconnection);

    let auth_service = AuthService::new(
        config.jwt.secret.clone(),
        config.jwt.access_token_expiry,
        config.jwt.refresh_token_expiry,
    );

    let auth_data = web::Data::new(auth_service);

    let host = config.server.host.clone();
    let port = config.server.port;

    println!("server is Ready and starting http://{}:{}", host, port);
    
    HttpServer::new(move || {
        let cors: Cors = Cors::default()
            .allowed_origin(&config.server.frontend_url)
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .expose_headers(vec!["Authorization"])
            .supports_credentials()
            .max_age(3600);

        App::new()
        .wrap(Logger::default())
        .wrap(cors)
        .app_data(db_data.clone())
        .app_data(auth_data.clone())
        .app_data(config_data.clone())
        .service(
                web::scope("/api/auth")
                    .route("/register", web::post().to(handlers::register))
                    .route("/login", web::post().to(handlers::login))
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}