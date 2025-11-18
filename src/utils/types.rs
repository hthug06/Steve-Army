use bytes::{Buf, BytesMut};

const MAX_VARINT_SIZE: i32 = 5;
const DATA_BITS_MASK: i32 = 127;
const CONTINUATION_BIT_MASK: i32 = 128;
const DATA_BITS_PER_BYTE: i32 = 7;
pub struct Varint;

impl Varint {
    pub fn get_bytes_size(value: i32) -> i32 {
        for i in 1..MAX_VARINT_SIZE {
            if (value & -1 << i * DATA_BITS_PER_BYTE) == 0 {
                return i;
            }
        }

        MAX_VARINT_SIZE
    }

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


    pub async fn write(output: &mut Vec<u8>, mut value: i32) { ;
        loop {
            let b: u8 = value as u8 & 0x7F;
            value >>= 7;
            if value == 0 {
                output.push(b);
                break;
            }
            output.push(b | 0x80);
        }
    }
}