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

    impl Encodable for i64 {
        fn encode<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
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
    use uuid::Uuid;

    impl Encodable for Uuid {
        fn encode<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
            writer.write(self.as_bytes())?;
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

#[cfg(test)]
mod test {
    use crate::packets::login::LoginRequest;
    use uuid::Uuid;

    #[test]
    fn login_request_test() {
        let request = LoginRequest {
            id: 3,
            uuid: Uuid::new_v4(),
            username: String::from("NV6"),
        };

        let buf = &mut Vec::<u8>::new();
        request.encode(buf).unwrap();

        println!("{:?}, {:?}", buf, request);
    }
}
