use std::io;
use std::io::{Cursor, Read, Write};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use crate::network::packets::handshake::intention::Intent;
use crate::network::packets::{Packet, RawPacket};

pub struct Client{
    reader: tokio::io::BufReader<tokio::net::tcp::OwnedReadHalf>,
    writer: tokio::io::BufWriter<tokio::net::tcp::OwnedWriteHalf>,
    state: Intent,
    gameprofile: Option<GameProfile>,
}

pub struct GameProfile {
    uuid: String,
    username: String,
    properties: GameProfileProperties
}

pub struct GameProfileProperties {
    name: String,
    value: String,
    signature: Option<String>
}

async fn read_varint_async<R: AsyncRead + Unpin>(reader: &mut R) -> io::Result<i32> {
    let mut value: i32 = 0;
    let mut position: i32 = 0;
    let mut current_byte: u8;

    loop {
        let mut buffer = [0; 1];
        reader.read_exact(&mut buffer).await?;
        current_byte = buffer[0];

        value |= ((current_byte & 0x7F) as i32) << position;

        if (current_byte & 0x80) == 0 {
            break;
        }

        position += 7;

        if position >= 32 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "VarInt est trop grand"));
        }
    }
    Ok(value)
}

/// Écrit un VarInt dans n'importe quelle source qui implémente `Write`.
async fn write_varint_async<W: AsyncWrite + Unpin>(writer: &mut W, mut value: i32) -> io::Result<()> {
    loop {
        let mut temp = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            temp |= 0x80;
        }
        writer.write_all(&[temp]).await?;
        if value == 0 {
            break;
        }
    }
    Ok(())
}

impl Client{
    pub async fn connect(address: &str) -> io::Result<Self>{
        let stream = tokio::net::TcpStream::connect(address).await?;
        let (read_half, write_half) = stream.into_split();

        Ok(Self {
            reader: tokio::io::BufReader::new(read_half),
            writer: tokio::io::BufWriter::new(write_half),
            state: Intent::Status,
            gameprofile: None
        })
    }

    pub async fn read_packet(&mut self) -> io::Result<RawPacket> {
        // 1. Lire la longueur totale du paquet (ID + Données)
        let packet_length = read_varint_async(&mut self.reader).await?;

        if packet_length <= 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Longueur de paquet invalide"));
        }

        // 2. Lire le reste du paquet (ID + Données) dans un buffer
        let mut packet_buffer = vec![0; packet_length as usize];
        self.reader.read_exact(&mut packet_buffer).await?;

        // 3. Utiliser un Cursor pour lire depuis ce buffer en mémoire
        // C'est plus facile que de lire directement depuis le flux réseau
        let mut cursor = Cursor::new(packet_buffer);

        // 4. Lire le Packet ID depuis le buffer
        let id = read_varint_async(&mut cursor).await?;

        // 5. Le reste du buffer contient les données
        let mut data = Vec::new();
        std::io::Read::read_to_end(&mut cursor, &mut data)?; // Lit de la position actuelle jusqu'à la fin

        Ok(RawPacket { id, data })
    }

    pub async fn send_packet(&mut self, packet: RawPacket) -> io::Result<()> {
        // 1. Créer un buffer en mémoire pour (ID + Données)
        let mut packet_buffer = Vec::new();

        // 2. Écrire le Packet ID dans le buffer
        write_varint_async(&mut packet_buffer, packet.id).await?;

        // 3. Écrire les Données dans le buffer
        std::io::Write::write_all(&mut packet_buffer, &packet.data)?;

        // 4. Obtenir la longueur totale (ID + Données)
        let packet_length = packet_buffer.len() as i32;

        // 5. Écrire la longueur totale (en VarInt) sur le VRAI flux réseau
        write_varint_async(&mut self.writer, packet_length).await?;

        // 6. Écrire le buffer (ID + Données) sur le VRAI flux réseau
        self.writer.write_all(&packet_buffer).await?;

        // 7. Vider le buffer d'écriture pour envoyer les données immédiatement
        self.writer.flush().await?;

        Ok(())
    }
}