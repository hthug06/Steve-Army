mod client;
mod network;
mod server_info;
mod utils;

use crate::client::Client;
use crate::network::packets::ServerPacket;
use crate::network::packets::handshake::{Intent, Intention};
use crate::network::packets::status::status_request::StatusRequest;
use crate::server_info::ServerInfo;
use clap::{Parser, arg};
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use tokio::task;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Address of the server
    #[arg(long)]
    adress: String,

    /// Port of the server
    #[arg(short, default_value_t = 25565)]
    port: u16,

    /// Show info of the server (imitate the server list)
    #[arg(short, default_value_t = false)]
    info: bool,
}

#[tokio::main]
async fn main() {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap(); //Start log
    let args = Args::parse();
    let mut join_handles = Vec::new();

    if args.info {
        ServerInfo::info(args.adress.clone(), args.port);
    } else {
        for num in 0..1 {
            let mut address_for_task = args.adress.clone();
            address_for_task.push_str(":");
            address_for_task.push_str(args.port.to_string().as_str());
            let addr = args.adress.clone();
            let handle = task::spawn(async move {
                log::info!("Connecting to {} for client #{}", address_for_task, num);
                match Client::connect(&address_for_task).await {
                    Ok(mut client) => {
                        log::info!("Client #{} is connected!", num);
                        // Send the first packet (handshake) + another one
                        client
                            .send_packet(
                                Intention::new(773, &addr, args.port, Intent::Login)
                                    .as_raw_packet()
                                    .await,
                            )
                            .await
                            .expect("failed to send Handshake packet");

                        client
                            .send_packet(StatusRequest.as_raw_packet().await)
                            .await
                            .unwrap();

                        // read loop for this client
                        loop {
                            match client.read_packet().await {
                                Ok(packet) => {
                                    //This is for debugging
                                    println!("{:?}", packet);

                                    match packet.id {
                                        // 0x0 => client.send_packet(PingRequest::default().as_raw_packet().await).await.expect("Failed to send ping Request packet"),
                                        _ => break,
                                    }
                                }
                                Err(e) => {
                                    log::error!("[Client {}] Erreur: {}", num, e);
                                    break;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Connexion failed for client #{} : {}", num, e);
                    }
                }
            });
            join_handles.push(handle);
        }

        for handle in join_handles {
            let _ = handle.await;
        }
    }

    //For finish
    tokio::signal::ctrl_c().await.unwrap();
    log::info!("Shutting down...");
}
