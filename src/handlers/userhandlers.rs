use std::{fmt::format, net::IpAddr};

use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, http::header::RETRY_AFTER, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web::cookie::{Cookie, SameSite, time::Duration};
use chrono::Local;
use lettre::transport::smtp::client;
use serde_json::{ser::CharEscape::CarriageReturn, value::to_raw_value};
use sqlx::types::ipnetwork::IpNetwork;
use uuid::{Uuid, Version::SortRand};
use validator::Validate;


use crate:: {
    auth::{AuthService, Claims},
    config::Config,
    db::database::Database,
    models::{
        user::{AuthResponse, ChangePasswordRequest, ForgotPasswordRequest, LoginRequest, LogoutRequest,
        RefreshToken, RegisterRequest, ResetPasswordRequest, UserResponse},
        apiResponse::{ApiResponse}
    },
};
 

pub async fn register(
    db: web::Data<Database>, 
    auth_Service: web::Data<AuthService>, 
    req: web::Json<RegisterRequest> ) -> impl Responder {
        println!("\n\n[ UserHandler ] :::>>> Register function in handlers.rs is being called by Main.rs...");

        let password_hash = match AuthService::hash_password(&req.password) {
            Ok(hash) => hash, 
            Err(_) => {
                return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Failed to hash password And Try again".to_string()
            ));
             }
        };

        let user = match db.create_user(
            &req.email, &req.username, &password_hash, &req.full_name
        ).await {
            Ok(user) => user, 
            Err(_) => {
                return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                    "Failed to Create User & try again".to_string()
                ));
            }
        };


        HttpResponse::Created().json(ApiResponse::success(UserResponse {
            id: user.id,
            email: user.email,
            username: user.username,
            full_name: user.full_name,
            is_verified: user.is_verified,
            created_at: user.created_at,

        }))
    }

pub async fn login(
    db: web::Data<Database>, 
    auth_Service: web::Data<AuthService>,
    config: web::Data<Config>,
    req: web::Json<LoginRequest>,
    http_req: HttpRequest, ) -> impl Responder {
        println!("\n\n[ UserHandler ] :::>>> login in handler.rs is being called by Main.rs");

        if let Err(errors) = req.validate() {

            println!("[ UserHandler ] :::>>> Validation fails in Login Phase : {}", errors);
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error (
                format!("Validation error : {}", errors)
            ));  
        }

        let clientIP: Option<String> = http_req
        .headers()
        .get("X-Forwarded-For")
        .and_then(|h: &actix_web::http::header::HeaderValue | h.to_str().ok())
        .map(|s| s.to_string())
        .or_else(|| {http_req.connection_info().realip_remote_addr().map(|s| s.to_string())});

        let ipNetwork: Option<IpNetwork> =clientIP.and_then(|ip_str| {
            let first_ip: &str = ip_str.split(',').next()?.trim();
            first_ip.parse::<IpAddr>().ok().map(IpNetwork::from)
        });

        if ipNetwork.is_none() {
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                "invalid Ip Network".to_string()
            ));
        }

        let user: crate::models::user::User = match db.get_user_by_email(&req.email).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                let resultOfRecordLogin = db.record_login_attempt(&req.email, ipNetwork.as_ref(), false).await;
                return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                    "Invalid Credentials".to_string()
                ));
            }
            Err(resultOfRecordLogin) => {
                return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                    "Database error".to_string()
                ));
            }
    };

    if !user.is_active {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            "Account is Deactivated".to_string()
        ));
    }

    match AuthService::verify_password(&req.password, &user.password_hash) {
        Ok(true) => {
            let _none1 = db.record_login_attempt(&req.email, ipNetwork.as_ref(), true).await;
            let _none2 = db.update_lastLogin(user.id).await;
        }
        Ok(false) => {
            let _none3 = db.record_login_attempt(&req.email, ipNetwork.as_ref(), false);
            return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                "Invalid Credintials".to_string()
            ));
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Error in verifying passowrd".to_string()
            ))
        }
    }

    
    let access_token = match auth_Service.generate_access_token(&user.id.to_string(), &user.email){
        Ok(token) => token,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "failed to generete access token".to_string()
            ));
        }
    };

    let cookie = Cookie::build("access_token", access_token.clone())
        .path("/")
        .http_only(false)
        .same_site(SameSite::Lax)
        .secure(true) 
        .max_age(Duration::days(2))  //config.jwt.access_token_expiry as i64
        .finish();

    println!("[ UserHandler ] :::>>> User {} is loggedIn..", &user.username);
    HttpResponse::Ok()
    .cookie(cookie)
    .json(ApiResponse::success(AuthResponse {
        access_token,
        token_type: "Bearer".to_string(),
        expires_in: config.jwt.access_token_expiry *60, 
        user: UserResponse { id: user.id, email: user.email, username: user.username, full_name: user.full_name, is_verified: user.is_verified, created_at: user.created_at,},
    }))
    
}