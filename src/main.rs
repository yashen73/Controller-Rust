mod auth;
mod config;
mod db;
mod handlers;
mod models;
mod dto;
mod ssh;
mod newerrors;

use actix_web::{App, HttpResponse, HttpServer, Responder, web::{self, Json}, middleware::Logger };
use actix_cors::Cors;
use actix_web_httpauth::middleware::HttpAuthentication;

use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use dotenv::dotenv;
use std::{env, fmt::format, ptr::null};


use crate::{
    auth::AuthService, 
    config::Config, 
    ssh::connectViaSSH,
    db::{database::Database, device_database::DeviceDatabase}, 
    handlers::{device_handler, ssh_handler, userhandlers}
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("infor"));

    dotenv().ok();

    let config = Config::from_env();
    let config_data = web::Data::new(config.clone());

    println!("\n\nTrying to connect User Database ....");
    let databaseconnection_1 = match Database::new(&config).await {
        Ok(db) => {
            println!("User Database Connected Successfully...");
            db
        }
        Err(e) => {
            eprintln!("Failed to connect User Database : {}", e);
            std::process::exit(1);
        }
    };

    println!("\n\nTrying to connect Device Database ....");
    let databaseconnection_2 = match DeviceDatabase::new(&config).await {
        Ok(db) => {
            println!("Device Database Connected Successfully...");
            db
        }
        Err(e) => {
            eprintln!("Failed to connect Device Database : {}", e);
            std::process::exit(1);
        }
    };

    let db_data_1 = web::Data::new(databaseconnection_1);
    let db_data_2 = web::Data::new(databaseconnection_2);

    let auth_service = AuthService::new(
        config.jwt.secret.clone(),
        config.jwt.access_token_expiry,
        config.jwt.refresh_token_expiry,
    );
    let ssh_service = ssh::connectViaSSH::Connect_Via_SSH::new();

    let auth_data = web::Data::new(auth_service);
    let ssh_data = web::Data::new(ssh_service);

    let host = config.server.host.clone();
    let port = config.server.port;

    println!("server is Ready and starting http://{}:{}", host, port);
    
    HttpServer::new(move || {
        let cors: Cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:5173")
            .allowed_origin_fn(|origin, _req_head| {
                let origin_str = origin.to_str().unwrap_or("");
                origin_str == "http://localhost:3000" || origin_str == "http://127.0.0.1:3000"
            })
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
        .app_data(db_data_1.clone())
        .app_data(db_data_2.clone())
        .app_data(auth_data.clone())
        .app_data(ssh_data.clone())
        .app_data(config_data.clone())
        .service(
                web::scope("/api/auth")
                    .route("/login", web::post().to(handlers::userhandlers::login))
                    .route("/editUser/{usermail}", web::post().to(handlers::userhandlers::register))
                    .route("/deleteUser/{usermail}", web::post().to(handlers::userhandlers::register))
        )
        .service(
                web::scope("/device")
                    .route("/deviceRegister", web::post().to(handlers::device_handler::register_device))
                    .route("/getAllDevices", web::get().to(handlers::device_handler::getAllDevices))
                    .route("/getDeviceInfo/{deviceId}", web::get().to(handlers::device_handler::getDeviceInfo))
                    .route("/editDevice/{deviceId}", web::get().to(handlers::device_handler::register_device))
                    .route("/deleteDevice/{deviceId}", web::get().to(handlers::device_handler::register_device))
                    .route("/test", web::get().to(handlers::device_handler::test))
        )
        .service(
                web::scope("/connectDevice")
                    .route("/deviceCMD", web::post().to(handlers::ssh_handler::connect_ssh))
                    .route("/send/closingSession", web::post().to(handlers::ssh_handler::closingSession))
                    .route("/send/sendCommand", web::post().to(handlers::ssh_handler::push_cmd))
                    .route("/test", web::get().to(handlers::ssh_handler::test))
        )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}