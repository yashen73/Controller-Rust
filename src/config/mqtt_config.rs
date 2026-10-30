use dotenv::dotenv;
use serde::Deserialize;
use std::{env, time::Duration};

pub struct Mqtt_config {
    pub mqtt_username: String,
    pub mqtt_host: String,
    pub mqtt_password: String,
    pub webhook_url: String,
    pub mqtt_port: u16,
    pub mqtt_client_id: String,
    pub webhook_timeout: Duration
}

impl Mqtt_config {
    pub fn from_env() -> Result<Self, String> {
        dotenv().ok();

        Ok(Self {
           mqtt_host: env::var("SERVER_HOST")
                .map_err(|_| "MQTT_HOSTNAME is required".to_string())?,

            mqtt_username: env::var("MQTT_USERNAME")
                .map_err(|_| "MQTT_USERNAME is required".to_string())?,

            mqtt_password: env::var("MQTT_PASSWORD")
                .map_err(|_| "MQTT_PASSWORD is required".to_string())?,

            webhook_url: env::var("WEBHOOK_URL")
                .map_err(|_| "WEBHOOK_URL is required".to_string())?,

            mqtt_port: env::var("MQTT_PORT")
                .unwrap_or_else(|_| "1883".to_string())
                .parse::<u16>()
                .map_err(|_| "MQTT_PORT must be a valid port".to_string())?,

            mqtt_client_id: env::var("MQTT_CLIENT_ID")
                .unwrap_or_else(|_| "rust-device-backend".into()),

            webhook_timeout: Duration::from_secs(10),
        })
    }
}