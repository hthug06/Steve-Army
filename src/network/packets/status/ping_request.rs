use std::time::{SystemTime, UNIX_EPOCH};
use crate::network::packets::{Packet, ServerPacket};

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

impl ServerPacket for PingRequest{
    fn id(&self) -> i32 {
        1
    }

    async fn data(&self) -> Vec<u8> {
        self.timestamp.to_be_bytes().to_vec()
    }
}