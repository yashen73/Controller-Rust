use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;
use ipnetwork::IpNetwork;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Device{
    pub id:Uuid,
    pub nominatedid: String,
    pub userid: Uuid,
    pub userusername: String,
    pub category: String,
    pub model: String,
    pub username: String,
    pub hostname: String,
    pub port_no: i64,
    pub ip_addr: Option<IpNetwork>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct DeviceRegisterRequest {
    pub nominatedid: String,
    pub category: String,
    pub model: String,
    pub username: String,
    pub hostname: String,
    pub port_no: i64,
    pub ip_addr: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct DeviceEditRequest{
    pub category: String,
    pub model: String,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ConnectDeviceSSH {
    pub username: String,
    pub port_no: i64,
    pub ip_addr: Option<IpNetwork>,
}   

#[derive(Debug, Serialize)]
pub struct DeviceResponse {
    pub userid: Uuid,
    pub nomintatedid: String,
    pub userusername: String,
    pub category: String,
    pub model: String,
    pub username: String,
    pub hostname: String,
    pub port_no: i64,
    pub ip_addr: Option<IpNetwork>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}