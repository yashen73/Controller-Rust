use actix_web::{App, HttpResponse, HttpServer, Responder, web::{self, Json}};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use dotenvy::dotenv;
use std::{env, fmt::format};

// Models
#[derive(Serialize, sqlx::FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct CreateUser {
    name:String,
    email: String,
}

// Handlers 

//create 
async fn create_user(
    pool:web::Data<PgPool>,
    payload: web::Json<CreateUser>,
) -> impl Responder {
    let user = sqlx::query_as::<_,User>(
        "INSERT INTO practicetable (name, email) VALUES ($1,$2) RETURNING id, name, email"
    )
    .bind(&payload.name)
    .bind(&payload.email)
    .fetch_one(pool.get_ref())
    .await;

    match user {
        Ok(u) => {
            println!("successfully inserted : {:?}", json!(u));
            HttpResponse::Ok().json(u)
        },
        Err(e) => {
            println!("DB Error : {:?}", e);
            HttpResponse::InternalServerError().body(format!("DB errror: {:?}", e))
        },
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url: String = env::var("DATABASE_URL").expect("Database Url must be set");
    let pool = PgPool::connect(&db_url).await.expect("DB connection failed");

    println!("Server running on http:127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .route("/users", web::post().to(create_user))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}