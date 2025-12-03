use crate::network::packets::ServerPacket;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct PingRequest {
    pub timestamp: u128,
}

impl Default for PingRequest {
    fn default() -> Self {
        Self {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        }
    }
}

impl ServerPacket for PingRequest {
    fn id(&self) -> i32 {
        1
    }

    async fn data(&self) -> Vec<u8> {
        self.timestamp.to_be_bytes().to_vec()
    }
}
