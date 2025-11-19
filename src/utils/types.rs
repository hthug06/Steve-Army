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
                return out
            }
        }

        return out;
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