use std::f32::consts::E;

use actix_web::{HttpRequest, HttpResponse, Responder, http::header, web};
use jsonwebtoken::TokenData;
use sqlx::{database, types::ipnetwork::IpNetwork};

use crate::{
    auth::{AuthService, Claims}, config::Config, db::{database::Database, device_database::DeviceDatabase}, models::{
        apiResponse::{self, ApiResponse}, device::{ConnectDeviceSSH, Device, DeviceEditRequest, DeviceRegisterRequest, DeviceResponse}, user::User
    }
};

pub async fn register_device(
    db: web::Data<DeviceDatabase>,
    db1: web::Data<Database>,
    auth_service: web::Data<AuthService>,
    head_req: HttpRequest,
    req: web::Json<DeviceRegisterRequest>) -> impl Responder{
    println!("\n\n[ DeviceHandler ] :::>>> register_Device is called...");
    
    //ConnecDeviceSsH variable
    

    //catching headers 
    let catch_access_token: Option<&str> = head_req.headers().get("Authorization").and_then(|v| v.to_str().ok()); 
    
    //Consider if there is not a token & User...
    if let Some(token) = catch_access_token {
        println!("[ DeviceHandler ] :::>>> Token is :::: {}", token);
        //extracting user from access_token
        
        match auth_service.verify_token(token) {
            Ok(tokendata) => {
                let found_claims = tokendata.claims;
                println!("[ DeviceHandler ] :::>>> claims from token found {:?}", found_claims);

                //Grab user from usermail...
                match db1.get_user_by_email(&found_claims.email).await {
                    Ok(Some(newuser)) => {
                        //catch Ip Address as we need....
                        let ip: Option<IpNetwork> = req.ip_addr.as_ref().and_then(|s| s.parse().ok());
                        //creating device....
                        let saved_device: Device = match db.create_device(
                            newuser.id, &req.nominatedid,  &newuser.username, &req.category, &req.model, &req.username, 
                            &req.hostname, req.port_no, ip.as_ref()
                        )
                        .await {
                            Ok(device) => {
                                //Setting variables for SSH session....
                                let to_connect_info = ConnectDeviceSSH{
                                    username: device.username.clone(),
                                    ip_addr: device.ip_addr.clone(),
                                    port_no: device.port_no,
                                };
                                
                                //Sending Http Response back....
                                return HttpResponse::Created().json(ApiResponse::success(DeviceResponse {
                                        userid:device.id,
                                        nomintatedid: device.nominatedid,
                                        userusername: device.userusername,
                                        category: device.category,
                                        model: device.model,
                                        username: device.username,
                                        hostname: device.hostname,
                                        port_no: device.port_no,
                                        ip_addr: device.ip_addr,
                                        is_active: device.is_active,
                                        created_at: device.created_at,
                                        updated_at: device.updated_at,
                                    }))
                            },
                            Err(e) => {
                                eprintln!("[ DeviceHandler ] :::>>> device creation is failed: {e:?}");
                                return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                                    "Failed to create Device & try again".to_string()
                                ));
                            }
                        };

                    },
                    Ok(None) => {
                        return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                            "Invalid Credentials".to_string()
                        ))
                    }
                    Err(_) => {
                        return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                            "failed to find User".to_string()
                        ));
                    }
                };

            }
            Err(_) => {
                println!("\n\n[ DeviceHandler ] :::>>> Issue with token. Check the Token");
                return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                    "No Valid Token".to_string()
            ));
        }
        };
        
    
    }else {
        println!("[ DeviceHandler ] :::>>>  No Token can be caught....");
        return HttpResponse::Unauthorized().finish();
    }
}


pub async fn getAllDevices(
    head_req: HttpRequest,
    auth_service: AuthService,
    db_for_user: Database,
    db_for_devices: DeviceDatabase
) -> impl Responder {
    println!("[ DeviceHandler ] :::>>> GetAllDevices is calling...");

    let catch_access_token:Option<&str>= head_req.headers().get("Authorization").and_then(|v| v.to_str().ok());


    if let Some(token) = catch_access_token {
        print!("[ DeviceHandler ] :::>>> Token is :::: {}", token);

        let found_claims_for_getAllDevices = match auth_service.verify_token(token) {
            Ok(tokenData) => tokenData.claims,
            Err(_) => {
                return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                    "Internal Server Error".to_string()
                ))
            }
        };

        let find_user = match db_for_user.get_user_by_email(&found_claims_for_getAllDevices.email).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                    "user Not found".to_string()
                ))
            },
            Err(_) => {
                return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                    "Internal Server Error".to_string()
                ))
            }
        };
    }

    

    return HttpResponse::Found().finish();
}

pub async fn getDeviceInfo(
    path: web::Path<(u32, String)>,
    req: HttpRequest
) -> impl Responder {
    println!("[ DeviceHandler ] :::>>> GetDeviceInfo is calling...");
    return HttpResponse::Found().finish();
}


pub async fn test() -> impl Responder {
    println!("[ DeviceHandler ] :::>>> test is calling...");
    return HttpResponse::Found().finish();
}
