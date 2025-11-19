use std::time::{SystemTime, UNIX_EPOCH};
use crate::network::packets::packet::Packet;

pub struct PingRequest{
    pub timestamp: i64
}

impl Default for PingRequest {
    fn default() -> Self {
        Self{
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64,
        }
    }
}

impl Packet for PingRequest{
    fn id(&self) -> u8 {
        1
    }

    async fn data(&self) -> Vec<u8> {
        self.timestamp.to_be_bytes().to_vec()
    }
}