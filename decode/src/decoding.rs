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
    use byteorder::{BigEndian, ReadBytesExt};

    impl Decodable for String {
        fn decode<R: Read>(reader: &mut R) -> Result<Self, std::io::Error> {
            let string_len = reader.read_u32::<BigEndian>()?;
            let mut buf = Vec::with_capacity(string_len as usize);
            reader.read(&mut buf)?;

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
