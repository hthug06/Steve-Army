mod utils;

use std::net::TcpStream;
use std::process::exit;
use bytes::{Buf, BufMut, BytesMut};
use clap::Parser;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpSocket;
use crate::utils::types::Varint;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args{
    /// Adress of the server
    #[arg(long)]
    adress: String,

    /// Port of the server
    #[arg(short, long, default_value_t = 25565)]
    port: u16,
}

impl Args {
    pub fn adress_and_port_valid(&self) ->bool{
        //Localhost is localhost lol
        if self.adress.eq("localhost") {
            return true;
        }
        
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


    if !args.adress_and_port_valid() {
        log::error!("Invalid adress format");
        exit(1);
    }

    //Connect to the server
    let tcp_stream = tokio::net::TcpStream::connect(&adrr_port).await.unwrap();
    let (mut reader, mut writer) = tokio::io::split(tcp_stream);

    tokio::spawn(async move {
        loop {
            let mut buf = [0u8; 32];
            reader.read(&mut buf).await.unwrap();
            if buf != [0u8; 32] {
                println!("{:?}", std::str::from_utf8(&buf));
            }
        }
    });


    println!("{}", &adrr_port);

    tokio::spawn(async move {

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        let mut data = & mut vec![];
        //Protocol version
        Varint::write(&mut data, 773).await;

        //Server adress (size + adresse to u8)
        let adress_clone = args.adress.clone();
        data.push(adress_clone.len() as u8);
        data.append(&mut adress_clone.into_bytes());

        //Server Port
        data.extend_from_slice(&mut &args.port.clone().to_be_bytes().to_vec());

        //Intent
        Varint::write(&mut data, 1).await;

        //Packet id
        let mut packet_id = &mut vec![];
        Varint::write(&mut packet_id, 0).await;

        //Lenght
        let mut lenght = &mut vec![];
        Varint::write(&mut lenght, (packet_id.len() + data.len()) as i32).await;

        //Final
        let final_packet = &mut vec![];
        final_packet.append(&mut lenght);
        final_packet.append(&mut packet_id);
        final_packet.append(&mut data);

        //send all
        writer.write_all(&final_packet).await.unwrap();
        writer.flush().await.unwrap();
        println!("send");
    });


    //For finish
    tokio::signal::ctrl_c().await.unwrap();
    log::info!("Shutting down...");
}
