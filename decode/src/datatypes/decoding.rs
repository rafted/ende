use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};
use uuid::Uuid;

use crate::{packets::Decodable, VarInt};

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

impl Decodable for u8 {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, std::io::Error> {
        reader.read_u8()
    }
}

impl Decodable for i8 {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, std::io::Error> {
        reader.read_i8()
    }
}

impl Decodable for i16 {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, std::io::Error> {
        reader.read_i16::<BigEndian>()
    }
}

impl Decodable for i64 {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, std::io::Error> {
        reader.read_i64::<BigEndian>()
    }
}

impl Decodable for f32 {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, std::io::Error> {
        reader.read_f32::<BigEndian>()
    }
}

impl Decodable for f64 {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, std::io::Error> {
        reader.read_f64::<BigEndian>()
    }
}

impl Decodable for String {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, std::io::Error> {
        let string_len = reader.read_i8()?;

        let mut buf = Vec::with_capacity(string_len as usize);
        buf.resize(string_len as usize, 0);

        reader.read(&mut buf[..])?;

        String::from_utf8(buf).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}

impl Decodable for Uuid {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, std::io::Error> {
        let buffer: &mut [u8; 16] = &mut [0; 16];
        reader.read(buffer)?;

        Ok(Uuid::from_bytes(*buffer))
    }
}

impl Decodable for Option<Uuid> {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, std::io::Error> {
        let result = reader.read_u8()?;

        if result == 0x01 {
            return Ok(Some(Uuid::decode(reader)?));
        }

        Ok(None)
    }
}
