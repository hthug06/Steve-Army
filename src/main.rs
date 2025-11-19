mod utils;
mod network;

use clap::Parser;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::network::packets::handshake::intention::{Intent, Intention};
use crate::network::packets::packet::Packet;
use crate::network::packets::status::ping_request::PingRequest;
use crate::network::packets::status::status_request::StatusRequest;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args{
    /// Address of the server
    #[arg(long)]
    adress: String,

    /// Port of the server
    #[arg(short, long, default_value_t = 25565)]
    port: u16,
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


    /*if !args.adress_and_port_valid() {
        log::error!("Invalid adress format");
        exit(1);
    }*/

    //Connect to the server
    let tcp_stream = tokio::net::TcpStream::connect(&adrr_port).await.unwrap();
    let (mut reader, mut writer) = tokio::io::split(tcp_stream);

    tokio::spawn(async move {
        loop {
            let mut buf = [0u8; 100];
            reader.read(&mut buf).await.unwrap();
            if buf != [0u8; 100] {
                println!("Reader response: {:?}", &buf);
            }
        }
    });


    println!("{}", &adrr_port);

    tokio::spawn(async move {

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        for i in 0..1 {
            //Send intention
            Intention::new(
                773,
                args.adress.clone(),
                args.port,
                Intent::Status
            ).send(&mut writer)
                .await
                .expect("Intention send");

            println!("Intention n°{} sent", i);

            //Then send status request
            StatusRequest.send(&mut writer).await.expect("StatusRequest send");
            println!("StatusRequest n°{} sent", i);

            //Finally send ping request
            PingRequest::default().send(&mut writer).await.expect("PingRequest send");
            println!("PingRequest n°{} sent", i);

        }

        writer.shutdown().await.unwrap();
    });


    //For finish
    tokio::signal::ctrl_c().await.unwrap();
    log::info!("Shutting down...");
}
