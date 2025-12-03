use std::io;
use std::io::Error;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub struct Varint;

impl Varint {
    pub fn read(buffer: &Vec<u8>) -> i32 {
        let mut out = 0;
        let mut bytes = 0;
        let mut index = 0;

        while index < buffer.len() {
            let ins = buffer[index];
            index += 1;

            out |= ((ins & 0x7F) as i32) << (bytes * 7);
            bytes += 1;

            if bytes > 5 {
                panic!("Varint too big");
            }

            if (ins & 0x80) == 0 {
                return out;
            }
        }

        out
    }

    pub async fn write(buffer: &mut Vec<u8>, value: i32) -> Result<(), Error> {
        let mut uvalue = value as u32;
        loop {
            let b: u8 = uvalue as u8 & 0x7F;
            uvalue >>= 7;
            if uvalue == 0 {
                buffer.push(b);
                break;
            }
            buffer.push(b | 0x80);
        }

        Ok(())
    }

    pub async fn write_string(buffer: &mut Vec<u8>, value: String) -> Result<(), Error> {
        let bytes = value.as_bytes();
        Varint::write(buffer, bytes.len() as i32).await?;
        buffer.extend_from_slice(&value.into_bytes());

        Ok(())
    }

    pub async fn read_async<R: AsyncRead + Unpin>(reader: &mut R) -> io::Result<i32> {
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
                return Err(io::Error::new(io::ErrorKind::InvalidData, "VarInt too big"));
            }
        }
        Ok(value)
    }

    pub async fn write_async<W: AsyncWrite + Unpin>(
        writer: &mut W,
        mut value: i32,
    ) -> io::Result<()> {
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
}
