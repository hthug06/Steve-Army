pub mod handshake;
pub mod status;

#[derive(Debug)]
pub struct RawPacket {
    pub id: i32,
    pub data: Vec<u8>,
}
pub trait ServerPacket {
    fn id(&self) -> i32;

    async fn data(&self) -> Vec<u8>;

    async fn as_raw_packet(&self) -> RawPacket {
        RawPacket {
            id: self.id(),
            data: self.data().await,
        }
    }
}
