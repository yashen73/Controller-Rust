use crate::{
    models::mqtt_models::Command, mqtt::client::{MqttPublishError, publishcommand}, state::appstate::AppState,
};

pub fn valid_req(request: &Command) -> bool {
    !request.id.trim().is_empty() 
    && request.id.len() <= 100 
    && !request.action.trim().is_empty()
    && request.action.len() <= 100
}

pub async fn send_device_command(
    state: &AppState,
    device_id: &str,
    request: &Command, ) -> Result<(), MqttPublishError> {
        publishcommand(&state.mqtt, device_id, &request.id, &request.action).await
    }