use serde_json::{from_str, Value};
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use crate::Args;
use crate::network::packets::handshake::intention::{Intent, Intention};
use crate::network::packets::packet::Packet;
use crate::network::packets::status::ping_request::PingRequest;
use crate::network::packets::status::status_request::StatusRequest;
use crate::utils::types::Varint;
pub struct ServerInfo;

impl ServerInfo {
    pub async fn info(reader: ReadHalf<TcpStream>, mut writer: WriteHalf<TcpStream>, args: &Args) {

        Self::read(reader).await;

        //Send intention
        Intention::new(
            773,
            args.adress.clone(),
            args.port,
            Intent::Status
        ).send(&mut writer)
            .await
            .expect("Intention not send");

        //Then send status request
        StatusRequest.send(&mut writer).await.expect("StatusRequest not send");

        //Finally send ping request (useless)
        PingRequest::default().send(&mut writer).await.expect("PingRequest not send");


    writer.shutdown().await.unwrap();
    }


    async fn read(mut reader: ReadHalf<TcpStream>) {
        tokio::spawn(async move {
            loop {
                //Create a buffer for the packet lenght only
                let mut buffer = vec![0; 2];
                reader.read(&mut buffer).await.unwrap();
                if buffer != [0; 2] {
                    let packet_size = Varint::read(&mut buffer);

                    //get the packet id after
                    buffer = vec![0; 1];
                    reader.read(&mut buffer).await.unwrap();
                    let packet_id = Varint::read(&mut buffer);

                    //Get the rest of the packet if this is the server info packet
                    if packet_id == 0x00{
                        buffer = vec![0; (packet_size-1) as usize];
                        reader.read(&mut buffer).await.unwrap();
                        ServerInfo::read_server_info(buffer).await;
                        break
                    }
                }
            }
        });
    }

    async fn read_server_info(buffer: Vec<u8>) {
        //Delete str len + something?
        let json = buffer[2..].to_vec();

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