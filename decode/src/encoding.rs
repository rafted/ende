use std::io::Write;

pub trait Encodable {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error>;
}

pub enum EncodingError {
    NotEnoughData(String),
}

mod bytes {
    use super::*;

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
}

mod floats {
    use super::Encodable;

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
}

mod string {
    use super::*;

    impl Encodable for String {
        fn encode<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
            writer.write(&[self.len() as u8])?;
            writer.write(self.as_bytes())?;
            Ok(())
        }
    }
}

mod uniqueid {
    use super::*;
    use byteorder::WriteBytesExt;
    use uuid::Uuid;

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
}

mod varint {
    use super::*;
    use crate::VarInt;

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
}

mod position {
    use crate::position::Position;

    use super::*;

    impl Encodable for Position {
        fn encode<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
            let value: i64 = ((self.x & 0x3FFFFFF) << 38) as i64
                | ((self.z & 0x3FFFFFF) << 12) as i64
                | (self.y as i32 & 0xFFF) as i64;

            let buf = value.to_be_bytes();

            writer.write_all(&buf)
        }
    }
}

mod animation {
    use byteorder::WriteBytesExt;

    use crate::packets::play::clientbound::animation::EntityAnimationType;

    use super::Encodable;

    impl Encodable for EntityAnimationType {
        fn encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
            match self {
                EntityAnimationType::SwingMainArm => writer.write_u8(0x0),
                EntityAnimationType::TakeDamage => writer.write_u8(0x1),
                EntityAnimationType::LeaveBed => writer.write_u8(0x2),
                EntityAnimationType::SwingOffHand => writer.write_u8(0x3),
                EntityAnimationType::CriticalEffect => writer.write_u8(0x4),
                EntityAnimationType::MagicCriticalEffect => writer.write_u8(0x5),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::{io::Cursor, str::FromStr};

    use crate::{decoding::Decodable, encoding::Encodable, packets::login::LoginRequest};
    use uuid::Uuid;

    #[test]
    fn login_request_test() {
        let unique_id = Some(Uuid::from_str("2fc8417d-9e8b-470b-9b73-aaa14fe177bc").unwrap());

        let request = LoginRequest {
            id: 0x00,
            uuid: unique_id,
            username: String::from("NV6"),
        };

        let buf = &mut Vec::<u8>::new();
        let buf2 = &mut Vec::<u8>::new();
        request.encode(buf).unwrap();
        unique_id.encode(buf2).unwrap();

        let mut cursor = Cursor::new(&buf);
        let mut cursor2 = Cursor::new(&buf2);

        assert_eq!(request, LoginRequest::decode(&mut cursor).unwrap());
        assert_eq!(Option::<Uuid>::decode(&mut cursor2).unwrap(), unique_id);
    }
}
