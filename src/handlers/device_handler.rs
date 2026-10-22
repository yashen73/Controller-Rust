use std::f32::consts::E;

use actix_web::{HttpRequest, HttpResponse, Responder, dev, http::header, web::{self, head}};
use jsonwebtoken::TokenData;
use lettre::transport::smtp::commands::{Auth, Data};
use sqlx::{database, types::ipnetwork::IpNetwork};

use crate::{
    auth::{self, AuthService, Claims}, config::Config, db::{database::Database, device_database::DeviceDatabase}, models::{
        apiResponse::{self, ApiResponse}, device::{ConnectDeviceSSH, Device, DeviceCard, DeviceEditRequest, DeviceRegisterRequest, DeviceResponse}, user::User
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
                            &req.hostname, req.port_no, ip.as_ref(), 0
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
    auth_service: web::Data<AuthService>,
    db_for_user: web::Data<Database>,
    db_for_devices: web::Data<DeviceDatabase>
) -> impl Responder {
    println!("\n\n[ DeviceHandler ] :::>>> GetAllDevices is calling...");

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
            Ok(Some(newuser)) => newuser,
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

        let find_devices: Vec<DeviceCard> = match db_for_devices.find_devices_by_userid(find_user.id).await {
            Ok(device_cards) => {
                if device_cards.is_empty() {
                    println!("[ DeviceHandler ] :::>>>  No Devices can be found");
                    return HttpResponse::NoContent().json(ApiResponse::<()>::error(
                        "No devices under this user".to_string()
                    ));
                }else {
                    println!("[ DeviceHandler ] :::>>> Found Some Devices");
                    return HttpResponse::Ok().json(ApiResponse::<>::success(device_cards));
                }
            },
            Err(_) => {
                return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                    "Internal Server Error".to_string()
                ))
            }
        };
    }

    return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
        "No access token is found".to_string()
    ));
}

pub async fn getDeviceInfo(
    path: web::Path<(String)>,
    head_req: HttpRequest,
    auth_Service: web::Data<AuthService>,
    db1_for_getdeviceinfo: web::Data<DeviceDatabase>
) -> impl Responder {
    println!("\n\n[ DeviceHandler ] :::>>> GetDeviceInfo is calling...");

    let(deviceId) = path.into_inner();
    let catch_access_troken: Option<&str>  = head_req.headers().get("Authorization").and_then(|v| v.to_str().ok());
    
    if let Some(token) = catch_access_troken{
        println!("[ DeviceHandler ] :::>>> Token is :::: {}", token);
        
        let found_claims_for_getdeviceinfo =  match auth_Service.verify_token(token){
            Ok(tokendata) => tokendata.claims,
            Err(_) => {
                println!("this is the error");
                return HttpResponse::NonAuthoritativeInformation().json(ApiResponse::<()>::error(
                    "No valid Information".to_string()
                ))
            }
        };
    }else {
        return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
            "Can not accept this request".to_string()
        ))
    }

    let device_card: Vec<DeviceCard> = match db1_for_getdeviceinfo.find_device_by_id(&deviceId.to_string()).await {
        Ok(found_device_card) => {
            println!("\nthe device is : {:?}", found_device_card);
            if found_device_card.is_empty(){
                return HttpResponse::NotFound().json(ApiResponse::<()>::error(
                "No Device Found".to_string()
                ))
            }else{
                println!("the device is : {:?}", found_device_card);
                return HttpResponse::Ok().json(ApiResponse::<>::success(found_device_card))
            }
        },
        Err(_) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Internal Server Error".to_string()
            ))
        }
    };

}


pub async fn test() -> impl Responder {
    println!("[ DeviceHandler ] :::>>> test is calling...");
    return HttpResponse::Found().finish();
}
