use crate::network::packets::packet::Packet;

pub struct StatusRequest;

impl Packet for StatusRequest {
    fn id(&self) -> u8 {
        0
    }

    async fn data(&self) -> Vec<u8> {
        Vec::new()
    }
}