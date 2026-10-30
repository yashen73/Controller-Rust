use std::{f32::consts::E, str::FromStr};

use actix_web::{HttpRequest, HttpResponse, Responder, dev, http::{self, header}, web::{self, Json, head}};
use actix_web_httpauth::middleware::HttpAuthentication;
use jsonwebtoken::{Header, TokenData};
use lettre::transport::smtp::commands::{Auth, Data};
use sqlx::{database, types::ipnetwork::IpNetwork};
use uuid::{NonNilUuid, Uuid};

use crate::{
    auth::{self, AuthService, Claims}, config::general_config, db::{database::Database, device_database::DeviceDatabase}, models::{
        apiResponse::{self, ApiResponse}, 
        device::{ Device, DeviceCard, DeviceEditRequest, DeviceRegisterRequest, DeviceResponse}, 
        sshConnection::{ConnectSSHRequest, ConnetedClosedSDeviceResponse, DeviceCMD, DeviceRPLY, CloseSSHRequest}, user::User
    }, ssh::{self, connectViaSSH::{self, Connect_Via_SSH}}
};

pub async fn connect_ssh(
    auth_service: web::Data<AuthService>,
    ssh_service: web::Data<Connect_Via_SSH>,
    req_header: HttpRequest,
    config: web::Data<general_config::Config>,
    req: Json<ConnectSSHRequest>,
    db_1: web::Data<Database>,
    db_2: web::Data<DeviceDatabase>) -> impl Responder{
    println!("\n\n[ SshHandler ] :::>>> connect_via_ssh is called...");
    
    //catching headers 
    let catch_access_token = req_header.headers().get("Authorization").and_then(|v| v.to_str().ok()); 
    
    if let Some(token) = catch_access_token {
        let tokenData = match auth_service.verify_token(token){
            Ok(tokenData ) => tokenData,
            Err(e) => {
            return HttpResponse::NotAcceptable().json(ApiResponse::<()>::error(
                "Not Acceptable Person".to_string()
            ));
            }
        };

        let extracted_user = match db_1.get_user_by_email(&tokenData.claims.email).await{
            Ok(Some(extracted_user)) => extracted_user,
            Ok(None) => {
                return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                    "Invalid credentials inside the token".to_string()
                ))
            },
            Err(e) => {
                return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                    "No valid Claims inside Token".to_string()
                ))
            }
        };

        let cmdpush = match ssh_service.connecting_device(req.into_inner()).await {
        Ok(cmdpush) => { 
            return HttpResponse::Created().json(ApiResponse::<>::success(
                "Succeed".to_string()
            ))
        },
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                e.to_string()
            ))
        }
    };
        
    }else {
        return HttpResponse::NotFound().json(ApiResponse::<()>::error(
            "Can not extract the person from Token".to_string()
        ))
    }
}

pub async fn push_cmd (
    auth_service: web::Data<AuthService>,
    ssh_service: web::Data<Connect_Via_SSH>,
    config: web::Data<general_config::Config>,
    req_header: HttpRequest,
    req: web::Json<DeviceCMD>,
    db_1: web::Data<Database>,
    db_2: web::Data<DeviceDatabase> ) -> impl Responder {

    //catching headers 
    let catch_access_token = req_header.headers().get("Authorization").and_then(|v| v.to_str().ok()); 
    
    if let Some(token) = catch_access_token {
        let tokenData = match auth_service.verify_token(token){
            Ok(tokenData ) => tokenData,
            Err(e) => {
            return HttpResponse::NotAcceptable().json(ApiResponse::<()>::error(
                "Not Acceptable Person".to_string()
            ));
            }
        };

        let extracted_user = match db_1.get_user_by_email(&tokenData.claims.email).await{
            Ok(Some(extracted_user)) => extracted_user,
            Ok(None) => {
                return HttpResponse::NotFound().json(ApiResponse::<()>::error(
                    "Invalid credentials inside the token".to_string()
                ))
            },
            Err(e) => {
                return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                    "No valid Claims inside Token".to_string()
                ))
            }
        };

         let pushcommand =  match ssh_service.push_command(req.into_inner()).await{
            Ok(pushcommand) => pushcommand,
            Err(e) => {
                return HttpResponse::NotFound().json(ApiResponse::<()>::error(
                    e.to_string()
                ));
            }
        };

        return HttpResponse::Ok().json(ApiResponse::success(
            pushcommand
        ));
        
    }else {
        return HttpResponse::NotFound().json(ApiResponse::<()>::error(
            "Can not extract the person from Token".to_string()
        ))
    }
}

pub async fn closingSession(
    auth_service: web::Data<AuthService>,
    ssh_service: web::Data<Connect_Via_SSH>,
    req_header: HttpRequest,
    req: web::Json<CloseSSHRequest>,
    db_1: web::Data<Database>,
    db_2: web::Data<DeviceDatabase> ) -> impl Responder {
    println!("\n\n[ ssh_handler ] :::>>> test is calling...");
    
    let payload = req.into_inner();

    //catching headers 
    let catch_access_token = req_header.headers().get("Authorization").and_then(|v| v.to_str().ok()); 
    
    if let Some(token) = catch_access_token {
        let tokenData = match auth_service.verify_token(token){
            Ok(tokenData ) => tokenData,
            Err(e) => {
            return HttpResponse::NotAcceptable().json(ApiResponse::<()>::error(
                "Not Acceptable Person".to_string()
            ));
            }
        };

        let extracted_user = match db_1.get_user_by_email(&tokenData.claims.email).await{
            Ok(Some(extracted_user)) => extracted_user,
            Ok(None) => {
                return HttpResponse::NotFound().json(ApiResponse::<()>::error(
                    "Invalid credentials inside the token".to_string()
                ))
            },
            Err(e) => {
                return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                    "No valid Claims inside Token".to_string()
                ))
            }
        };

         let closing = match ssh_service.closing_ssh_session(payload.clone()).await{
            Ok(closing_result) => {
                return HttpResponse::Created().json(ApiResponse::<>::success(payload));
            },
            Err(e) => {
                return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                    e.to_string()
                ));
            }
        };

    return HttpResponse::Ok().finish();
        
    }else {
        return HttpResponse::NotFound().json(ApiResponse::<()>::error(
            "Can not extract the person from Token".to_string()
        ))
    }

    
}


pub async fn test(
    auth_service: web::Data<AuthService>,
    ssh_service: web::Data<Connect_Via_SSH> ) -> impl Responder {
    println!("\n\n[ ssh_handler ] :::>>> test is calling...");
    return HttpResponse::Ok().finish();
}
