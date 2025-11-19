use tokio::io::{AsyncWriteExt, WriteHalf};
use tokio::net::TcpStream;
use crate::utils::types::Varint;

pub trait Packet{
    async fn length(&self) -> Vec<u8>{
        let mut buffer_id = Vec::new();
        Varint::write(&mut buffer_id, Self::id(self) as i32).await;
        vec![buffer_id.len() as u8 + Self::data(self).await.len() as u8]
    }
    fn id(&self) -> u8;
    async fn data(&self) -> Vec<u8>;

    async fn send(&self, writer: &mut WriteHalf<TcpStream>) -> std::io::Result<()>{
        let mut buffer = Vec::new();
        self.write_packet(&mut buffer).await;
        writer.write_all(buffer.as_slice()).await?;
        writer.flush().await?;

        Ok(())
    }

    async fn write_packet(&self, buffer: &mut Vec<u8>){
        buffer.extend(self.length().await);
        Varint::write(buffer, self.id() as i32).await;
        buffer.append(&mut self.data().await);
    }
}