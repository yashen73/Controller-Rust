
use rumqttc::AsyncClient;

#[derive(Clone)]
pub struct AppState {
    pub mqtt: AsyncClient,
}