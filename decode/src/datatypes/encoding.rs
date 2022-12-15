use std::io::Write;

use byteorder::WriteBytesExt;
use uuid::Uuid;

use crate::{packets::Encodable, VarInt};

impl Encodable for VarInt {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        let mut remaining = *self;
        while remaining >= 0b10000000 {
            let byte = (remaining as u8) | 0b10000000;

            writer.write_all(&[byte])?;
            remaining >>= 7;
        }
        let byte = remaining as u8;

        writer.write_all(&[byte])
    }
}

impl Encodable for u8 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        writer.write_all(&[*self])
    }
}

impl Encodable for i8 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        writer.write_all(&[*self as u8])
    }
}

impl Encodable for i16 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        writer.write_all(&self.to_be_bytes())
    }
}

impl Encodable for i64 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        writer.write_all(&self.to_be_bytes())
    }
}

impl Encodable for f32 {
    fn encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        writer.write_all(&self.to_be_bytes())
    }
}

impl Encodable for f64 {
    fn encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        writer.write_all(&self.to_be_bytes())
    }
}

impl Encodable for String {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        writer.write(&[self.len() as u8])?;
        writer.write(self.as_bytes())?;
        Ok(())
    }
}

impl Encodable for Uuid {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        writer.write(self.as_bytes())?;
        Ok(())
    }
}

impl Encodable for Option<Uuid> {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        let has_unique_id = if self.is_some() { 0x01 } else { 0x00 };
        writer.write_u8(has_unique_id)?;

        if let Some(id) = self {
            id.encode(writer)?;
        }

        Ok(())
    }
}
