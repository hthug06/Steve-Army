use bytes::{Buf, BytesMut};

const MAX_VARINT_SIZE: i32 = 5;
const DATA_BITS_MASK: i32 = 127;
pub struct Varint;

impl Varint {
    pub fn has_continuation_bit(value: i32) -> bool {
        (value & 128) == 128
    }

    pub fn read(mut input: BytesMut) -> i32 {
        let mut out = 0;
        let mut bytes = 0;

        let mut byte_in: i32;

        loop {
            byte_in = input.get_i32();
            bytes += 1;
            out |= (byte_in & DATA_BITS_MASK) << bytes * 7;

            if bytes > MAX_VARINT_SIZE {
                panic!("Varint too big");
            }

            if Self::has_continuation_bit(byte_in){
                break;
            }
        }
        out
    }


    pub async fn write(buffer: &mut Vec<u8>, mut value: i32) {
        loop {
            let b: u8 = value as u8 & 0x7F;
            value >>= 7;
            if value == 0 {
                buffer.push(b);
                break;
            }
            buffer.push(b | 0x80);
        }
    }

    pub async fn write_string(buffer: &mut Vec<u8>, value: String) {
        buffer.push(value.len() as u8);
        buffer.extend_from_slice(&value.into_bytes());
    }
}