use crate::network::packets::RawPacket;
use crate::network::packets::handshake::Intent;
use crate::utils::types::Varint;
use std::io;
use std::io::Cursor;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct Client {
    reader: tokio::io::BufReader<tokio::net::tcp::OwnedReadHalf>,
    writer: tokio::io::BufWriter<tokio::net::tcp::OwnedWriteHalf>,
    state: Intent,
    gameprofile: Option<GameProfile>,
}

pub struct GameProfile {
    uuid: String,
    username: String,
    properties: GameProfileProperties,
}

pub struct GameProfileProperties {
    name: String,
    value: String,
    signature: Option<String>,
}

impl Client {
    pub async fn connect(address: &str) -> io::Result<Self> {
        let stream = tokio::net::TcpStream::connect(address).await?;
        let (read_half, write_half) = stream.into_split();

        Ok(Self {
            reader: tokio::io::BufReader::new(read_half),
            writer: tokio::io::BufWriter::new(write_half),
            state: Intent::Status,
            gameprofile: None,
        })
    }

    pub async fn read_packet(&mut self) -> io::Result<RawPacket> {
        // Read total packet size
        let packet_length = Varint::read_async(&mut self.reader).await?;

        if packet_length <= 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Longueur de paquet invalide",
            ));
        }

        // Read the rest of the packet into a buffer
        let mut packet_buffer = vec![0; packet_length as usize];
        self.reader.read_exact(&mut packet_buffer).await?;

        let mut cursor = Cursor::new(packet_buffer);

        // Read packet id from buffer
        let id = Varint::read_async(&mut cursor).await?;

        // Read the rest of the buffer
        let mut data = Vec::new();
        std::io::Read::read_to_end(&mut cursor, &mut data)?;

        Ok(RawPacket { id, data })
    }

    pub async fn send_packet(&mut self, packet: RawPacket) -> io::Result<()> {
        // Create a packet following this: https://minecraft.wiki/w/Java_Edition_protocol/Packets#Packet_format

        let mut packet_buffer = Vec::new();

        // Put the packet id in the buffer
        Varint::write_async(&mut packet_buffer, packet.id).await?;

        //P ut data in the buffer
        std::io::Write::write_all(&mut packet_buffer, &packet.data)?;

        // Get total packet size
        let packet_length = packet_buffer.len() as i32;

        Varint::write_async(&mut self.writer, packet_length).await?;

        self.writer.write_all(&packet_buffer).await?;

        self.writer.flush().await?;

        Ok(())
    }
}
