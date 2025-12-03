use crate::network::packets::ServerPacket;

pub struct StatusRequest;

impl ServerPacket for StatusRequest {
    fn id(&self) -> i32 {
        0
    }

    async fn data(&self) -> Vec<u8> {
        vec![]
    }
}
