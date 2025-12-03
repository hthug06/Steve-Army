use crate::client::Client;
use crate::network::packets::handshake::{Intent, Intention};
use crate::network::packets::status::ping_request::PingRequest;
use crate::network::packets::status::status_request::StatusRequest;
use crate::network::packets::{RawPacket, ServerPacket};
use serde_json::{Value, from_str};
use tokio::task;

pub struct ServerInfo;

impl ServerInfo {
    pub fn info(address: String, port: u16) {
        let mut address_for_task = address.clone();
        address_for_task.push_str(":");
        address_for_task.push_str(port.to_string().as_str());
        task::spawn(async move {
            match Client::connect(&address_for_task).await {
                Ok(mut client) => {
                    // Send the first packet (handshake) + the status request
                    client
                        .send_packet(
                            Intention::new(773, &address, port, Intent::Status)
                                .as_raw_packet()
                                .await,
                        )
                        .await
                        .expect("Failed to send intention packet");

                    client
                        .send_packet(StatusRequest.as_raw_packet().await)
                        .await
                        .unwrap();

                    // read loop for this client
                    loop {
                        match client.read_packet().await {
                            Ok(packet) => match packet.id {
                                0x0 => {
                                    Self::handle_status_request(packet);
                                    client
                                        .send_packet(PingRequest::default().as_raw_packet().await)
                                        .await
                                        .expect("Failed to send ping Request packet")
                                }
                                _ => break,
                            },
                            Err(e) => {
                                log::error!("[{}] Error: {}", address_for_task, e);
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    log::error!("[{}] Connexion failed: {}", address_for_task, e);
                }
            }
        });
    }

    fn handle_status_request(raw_packet: RawPacket) {
        let json = raw_packet.data[2..].to_vec();

        match String::from_utf8(json) {
            Ok(json_str) => match from_str::<Value>(&json_str) {
                Ok(json_value) => {
                    if let Some(description) = json_value.get("description") {
                        log::info!("Description: {}", description);
                    }

                    if let Some(players) = json_value.get("players") {
                        if let Some(max) = players.get("max") {
                            log::info!("player: max - {}", max);
                        }
                        if let Some(online) = players.get("online") {
                            log::info!("player: online - {}", online);
                            if online.as_i64().unwrap() > 0 {
                                if let Some(sample) = players.get("sample") {
                                    for player_info in sample.as_array().unwrap() {
                                        log::info!("- {}", player_info["name"].as_str().unwrap());
                                    }
                                }
                            }
                        }
                    }

                    if let Some(version) = json_value.get("version") {
                        if let Some(name) = version.get("name") {
                            log::info!("Version name: {}", name);
                        }
                    }

                    if let Some(favicon) = json_value.get("favicon") {
                        if favicon.is_null() {
                            log::info!("No Favicon");
                        } else {
                            log::info!("The server have a favicon");
                        }
                    }

                    if let Some(chat) = json_value.get("enforcesSecureChat") {
                        log::info!("Enforce secure chat: {}", chat);
                    }

                    if let Some(chat) = json_value.get("enforce_secure_chat") {
                        log::info!("Enforce secure chat: {}", chat);
                    }
                }
                Err(e) => {
                    log::error!("Error while parsing JSON: {}", e);
                }
            },
            Err(e) => {
                log::error!("Error on UTF-8 conversion: {}", e);
            }
        }
    }
}
