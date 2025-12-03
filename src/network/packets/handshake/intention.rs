use crate::network::packets::{Packet, RawPacket, ServerPacket};
use crate::utils::types::Varint;

#[derive(Clone)]
pub enum Intent{
    Status = 1,
    Login = 2,
    Transfer = 3,
}

pub struct Intention {
    protocol_version: u16,
    server_adress: String,
    server_port: u16,
    intent: Intent,
}

impl Intention {
    pub fn new(protocol_version: u16, server_adress: &String, server_port: u16, intent: Intent) -> Self {
        Self{
            protocol_version,
            server_adress: server_adress.clone(),
            server_port,
            intent,
        }
    }
}

impl ServerPacket for Intention {

    fn id(&self) -> i32 {
        0
    }

    async fn data(&self) -> Vec<u8> {
        let mut data = Vec::new();

        //Protocol version
        Varint::write(&mut data, self.protocol_version as i32).await;

        //Server adress
        Varint::write_string(&mut data, self.server_adress.clone()).await;

        //Server Port
        Varint::write(&mut data, self.server_port as i32).await;

        //Intent
        Varint::write(&mut data, self.intent.clone() as i32).await;

        data
    }
}