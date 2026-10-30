use actix_web::{ HttpMessage, HttpRequest, HttpResponse, Responder, http::header::RETRY_AFTER, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web::cookie::{Cookie, SameSite, time::Duration};
use chrono::Local;
use lettre::transport::smtp::client;
use serde_json::{ser::CharEscape::CarriageReturn, value::to_raw_value};
use sqlx::types::ipnetwork::IpNetwork;
use uuid::{Uuid, Version::SortRand};
use validator::Validate;

use crate::{
    auth::{self, AuthService, Claims}, 
    config::{general_config, mqtt_config}, 
    db::{database::Database, device_database::DeviceDatabase}, 
    models::{
        mqtt_models::Command,
        apiResponse::{self, ApiResponse}, 
        device::{ Device, DeviceCard, DeviceEditRequest, DeviceRegisterRequest, DeviceResponse}, 
        sshConnection::{ConnectSSHRequest, ConnetedClosedSDeviceResponse, DeviceCMD, DeviceRPLY, CloseSSHRequest}, user::User
    },
    ssh::{self, connectViaSSH::{self, Connect_Via_SSH}},
    state::appstate::AppState,
    service::mqtt_service::{valid_req, send_device_command}
};

pub async fn sending_commands(
    auth: web::Data<AuthService>,
    state: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<Command>, ) -> impl Responder {
        let device_id = path.into_inner();
        

        if !valid_req(&body) {
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                "Can not accept this body".to_string()
            ));
        }

        match send_device_command(&state, &device_id, &body).await {
            Ok(()) => { 
                return HttpResponse::Accepted().json(ApiResponse::<>::success(
                    "accepted".to_string()
                ))
            },
            Err(e) => {
                return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                    "can not accept".to_string()
                ))
            }
        }
    }

pub async fn health_check(
    auth: web::Data<AuthService>
) -> impl Responder {
    return HttpResponse::Ok().finish();
}