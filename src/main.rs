mod utils;
mod network;
mod client;
mod send_steve;
mod server_info;

use clap::{arg, Parser};
use tokio::task;
use crate::client::Client;
use crate::network::packets::handshake::intention::{Intent, Intention};
use crate::network::packets::ServerPacket;
use crate::network::packets::status::ping_request::PingRequest;
use crate::network::packets::status::status_request::StatusRequest;
use crate::send_steve::SendSteve;
use crate::server_info::ServerInfo;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args{
    /// Address of the server
    #[arg(long)]
    adress: String,

    /// Port of the server
    #[arg(short, default_value_t = 25565)]
    port: u16,

    /// Show info of the server (imitate the server list)
    #[arg(short, default_value_t = false)]
    info: bool
}

impl Args {
    pub fn adress_and_port_valid(&self) ->bool{
        let split_addr = self.adress.split(".").collect::<Vec<&str>>();
        if split_addr.len() != 4 {
            return false;
        }

        //Check if address is in a valid format
        for addr in split_addr{
            if let Err(_) = addr.parse::<u8>(){
                return false;
            }
        }

        //No need to check for the port, it's already checked by the parser
        true
    }
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap(); //Start log
    let args = Args::parse();
    let mut join_handles = Vec::new();

    if args.info {
        ServerInfo::info(args.adress.clone(), args.port);
    }
    else{

        for _ in 0..1 {
            let mut address_for_task = args.adress.clone();
            address_for_task.push_str(":");
            address_for_task.push_str(args.port.to_string().as_str());
            let addr = args.adress.clone();
            println!("Connecting to {}", address_for_task);
            let handle = task::spawn(async move {
                println!("Connexion à {}", address_for_task);
                match Client::connect(&address_for_task).await {
                    Ok(mut client) => {
                        println!("Connecté !");
                        // Send the first packet (handshake) + another one
                        client.send_packet(Intention::new(
                            773,
                            &addr,
                            args.port,
                            Intent::Login,
                        ).as_raw_packet().await)
                            .await
                            .expect("idk");

                        client.send_packet(StatusRequest.as_raw_packet().await).await.unwrap();

                        // read loop for this client
                        loop {
                            match client.read_packet().await {
                                Ok(packet) => {
                                    println!("[{}] Reçu paquet: ID=0x{:X}", address_for_task, packet.id);
                                    println!("{:?}", packet);

                                    match packet.id {
                                        // 0x0 => client.send_packet(PingRequest::default().as_raw_packet().await).await.expect("Failed to send ping Request packet"),
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
