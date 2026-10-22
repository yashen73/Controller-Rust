use validator::Validate;
use ipnetwork::IpNetwork;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};


#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct ConnectDeviceSSH {
    pub id: Uuid,
    pub username: String,
    pub port_no: i64,
    pub ip_addr: Option<IpNetwork>
}   

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct ConnectDeviceRequest {
    pub id: Uuid,
    pub username: String,
    pub port_no: i64,
    pub ip_addr: Option<IpNetwork>,
    pub attempttime: DateTime<Utc>
}   

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConnetedSDeviceResponse{
    pub id: Uuid,
    pub username: String,
    pub port_no: i64,
    pub ip_addr: Option<IpNetwork>,
    pub attempttime: DateTime<Utc>,
    pub attempt: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceCMD{
    pub id: Uuid,
    pub username: String,
    pub hostname: String,
    pub port_no: i64,
    pub ip_addr: Option<IpNetwork>,
    pub cmd: String
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceRPLY{
    pub id: Uuid,
    pub username: String,
    pub hostname: String,
    pub port_no: i64,
    pub ip_addr: Option<IpNetwork>,
    pub rply: String
}