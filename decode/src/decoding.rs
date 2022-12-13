use std::io::Read;

pub trait Decodable: Sized {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, std::io::Error>;
}

pub enum DecodingError {
    NotEnoughData(String),
}

mod bytes {
    use super::*;
    use byteorder::ReadBytesExt;

    impl Decodable for u8 {
        fn decode<R: Read>(reader: &mut R) -> Result<Self, std::io::Error> {
            reader.read_u8()
        }
    }
}

mod strings {
    use super::*;
    use byteorder::ReadBytesExt;

    impl Decodable for String {
        fn decode<R: Read>(reader: &mut R) -> Result<Self, std::io::Error> {
            let string_len = reader.read_i8()?;

            let mut buf = Vec::with_capacity(string_len as usize);
            buf.resize(string_len as usize, 0);

            reader.read(&mut buf[..])?;

            String::from_utf8(buf)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
        }
    }
}

mod uniqueid {
    use super::*;
    use uuid::Uuid;

    impl Decodable for Uuid {
        fn decode<R: Read>(reader: &mut R) -> Result<Self, std::io::Error> {
            let buffer: &mut [u8; 16] = &mut [0; 16];
            reader.read(buffer)?;

            Ok(Uuid::from_bytes(*buffer))
        }
    }
}

mod varint {
    use super::*;
    use crate::VarInt;

    impl Decodable for VarInt {
        fn decode<R: Read>(reader: &mut R) -> Result<Self, std::io::Error> {
            let mut result = 0;
            let mut shift = 0;

            loop {
                let mut byte = [0];

                reader.read_exact(&mut byte)?;

                let value = (byte[0] & 0b01111111) as i32;

                result |= value << shift;
                shift += 7;

                if byte[0] & 0b10000000 == 0 {
                    break;
                }
            }
            
            Ok(result)
        }
    }
}


#[cfg(test)]
mod test {
    use crate::packets::login::LoginRequest;
    use std::io::Cursor;

    #[test]
    fn login_request_test() {
        let data = [
            3, 39, 87, 131, 24, 27, 146, 72, 22, 145, 21, 235, 167, 174, 177, 105, 118, 3, 78, 86,
            54,
        ];
        let cursor = &mut Cursor::new(&data);
        println!("{:?}, {:?}", data, LoginRequest::decode(cursor));
    }
}
