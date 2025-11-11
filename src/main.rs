use std::net::TcpStream;
use std::process::exit;
use clap::Parser;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpSocket;

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
    let tcp_stream = TcpStream::connect(&adrr_port)
        .expect("Couldn't connect to the server");

    println!("{}", &adrr_port);
    let stream = TcpStream::connect(adrr_port)
        .expect("Can't connect to the server");

    // TcpSocket::connect().await.expect("Can't connect to server").write().await;

    //For finish
    tokio::signal::ctrl_c().await.unwrap();
    log::info!("Shutting down...");
}
