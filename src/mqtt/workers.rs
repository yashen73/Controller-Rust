use reqwest::Client;
use rumqttc::{AsyncClient, Event, EventLoop, Packet, QoS};
use serde_json::Value;
use std::time::Duration;

pub async fn run_worker(
    client: AsyncClient,
    mut eventloop: EventLoop,
    webhook_url: String,
    http: Client, ) {
        if let Err(e) = client.subscribe("device/+/telemetry", QoS::AtLeastOnce).await{
            tracing::error!(error = %e, "Unqble to queue telemetry subscription");
        }

        if let Err(e) = client.subscribe("device/+/results", QoS::AtLeastOnce).await{
            tracing::error!(Error =%e, "unable to Queu results subsscription");
        }

        loop {
            match eventloop.poll().await {
                Ok(Event::Incoming(Packet::ConnAck(_))) => {
                    tracing::info!("Connected to mqtt broker");
                }
                Ok(Event::Incoming(Packet::Publish(message))) => {
                    let topic = message.topic.clone();

                    let data: Value = match serde_json::from_slice(&message.payload) {
                        Ok(value) => value,
                        Err(e) => {
                            tracing::warn!(
                                topic = %topic,
                                error = %e,
                                "invalid JSON received"
                            );
                            continue;
                        }
                    };

                    if topic.ends_with("/telemetry") {
                        tracing::info!(topic = %topic, "Telemetry received");

                        if let Err(e) = http.post(&webhook_url).json(&data).send().await
                        {
                            tracing::error!(
                                topic = %topic,
                                error = %e,
                                "Telemetry webhook delivered failed"
                            );
                        }
                    } else if topic.ends_with("/results") {
                        tracing::info!(
                            topic = %topic,
                            result = %data,
                            "Device command result recieved"
                        );
                    }
                }
                
                Ok(_) => {}
                
                Err(e) => {
                    tracing::error!(error = %e, "MQTT event-loop error");
                    tokio::time::sleep(Duration::from_secs(2)).await;
                }
            }
        }   

    }