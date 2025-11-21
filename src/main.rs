mod utils;
mod network;
mod client;
mod send_steve;
mod server_info;

use clap::Parser;
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
    let adrr_port = &format!("{}:{}", args.adress.parse::<String>().unwrap(), args.port);

    //Connect to the server
    let tcp_stream = tokio::net::TcpStream::connect(&adrr_port).await.unwrap();
    let (reader, writer) = tokio::io::split(tcp_stream);

    if args.info {
        ServerInfo::info(reader, writer, &args).await;
    }

    else{
        SendSteve::new(10);
    }

    //For finish
    tokio::signal::ctrl_c().await.unwrap();
    log::info!("Shutting down...");

}
