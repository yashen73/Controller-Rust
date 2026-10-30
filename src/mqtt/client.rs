use rumqttc::{AsyncClient, QoS};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MqttPublishError {
    #[error("Mqtt publish queue is unavailable")]
    PublishFailed,
}


#[derive(Serialize)]
struct DeviceCommand<'s> {
    id: &'s str,
    action: &'s str,
}

pub async fn publishcommand(
    client : &AsyncClient,
    device_id : &str,
    id: &str,
    action: &str, ) -> Result<(), MqttPublishError> {
        let topic  =  format!("device/{device_id}/commands");

        let payload = serde_json::to_vec(&DeviceCommand{id, action,}).map_err(|_| MqttPublishError::PublishFailed)?;

        client
            .publish(topic, QoS::AtLeastOnce, false, payload)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to enqueue MQTT command");
                MqttPublishError::PublishFailed
            })?;

            Ok(())
    }