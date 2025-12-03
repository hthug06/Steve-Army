use std::io;
use serde_json::{from_str, json, Value};
use tokio::task;
use crate::client::Client;
use crate::network::packets::handshake::intention::{Intent, Intention};
use crate::network::packets::{RawPacket, ServerPacket};
use crate::network::packets::status::ping_request::PingRequest;
use crate::network::packets::status::status_request::StatusRequest;

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
                    client.send_packet(Intention::new(
                        773,
                        &address,
                        port,
                        Intent::Status,
                    ).as_raw_packet().await)
                        .await
                        .expect("Failed to send intention packet");

                    client.send_packet(StatusRequest.as_raw_packet().await).await.unwrap();

                    // read loop for this client
                    loop {
                        match client.read_packet().await {
                            Ok(packet) => {
                                match packet.id {
                                    0x0 => {
                                        Self::handle_status_request(packet);
                                        client.send_packet(PingRequest::default().as_raw_packet().await).await.expect("Failed to send ping Request packet")
                                    },
                                    _ => break,
                                }
                            }
                            Err(e) => {
                                println!("[{}] Erreur: {}", address_for_task, e);
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[{}] Connexion échouée: {}", address_for_task, e);
                }
            }
        });
    }

    fn handle_status_request(raw_packet: RawPacket) {
        let json = raw_packet.data[2..].to_vec();

        match String::from_utf8(json) {
            Ok(json_str) => {
                match from_str::<Value>(&json_str) {
                    Ok(json_value) => {
                        // println!("{}", json_value);
                        if let Some(description) = json_value.get("description") {
                            println!("Description: {}", description);
                        }

                        if let Some(players) = json_value.get("players") {
                            if let Some(max) = players.get("max") {
                                println!("player: max - {}", max);
                            }
                            if let Some(online) = players.get("online") {
                                println!("player: online - {}", online);
                                if online.as_i64().unwrap() > 0 {
                                    if let Some(sample) = players.get("sample") {
                                        for player_info in sample.as_array().unwrap() {
                                            println!("- {}", player_info["name"].as_str().unwrap());
                                        }
                                    }
                                }
                            }
                        }

                        if let Some(version) = json_value.get("version") {
                            if let Some(name) = version.get("name") {
                                println!("Version name: {}", name);
                            }
                        }

                        if let Some(favicon) = json_value.get("favicon") {
                            if favicon.is_null() {
                                println!("No Favicon");
                            }
                            else{
                                println!("The server have a favicon");
                            }
                        }

                        if let Some(chat) = json_value.get("enforcesSecureChat") {
                            println!("Enforce secure chat: {}", chat);
                        }

                        if let Some(chat) = json_value.get("enforce_secure_chat") {
                            println!("Enforce secure chat: {}", chat);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error while parsing JSON: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error on UTF-8 conversion: {}", e);
            }
        }
    }
}