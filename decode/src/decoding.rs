use std::io::Read;

pub trait Decodable: Sized {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, std::io::Error>;
}

pub enum DecodingError {
    NotEnoughData(String),
}

mod bytes {
    use super::*;
    use byteorder::{BigEndian, ReadBytesExt};

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
}

mod floats {
    use byteorder::{BigEndian, ReadBytesExt};

    use super::*;

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
    use byteorder::ReadBytesExt;
    use uuid::Uuid;

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

mod position {
    use std::num::TryFromIntError;

    use byteorder::{BigEndian, ReadBytesExt};

    use crate::position::Position;

    use super::Decodable;

    impl Decodable for Position {
        fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, std::io::Error> {
            let value = reader.read_i64::<BigEndian>()?;

            // we technically don't need this function here, but it's easier like this so we can handle the error mapping in a single call - instead of doing it for every single try_into() statement,
            // we also could implement From<TryFromIntError> into std::io::Error, but that's a lot of work, and I think this works just as fine.
            fn calc_x_y_z(value: i64) -> Result<(i32, i16, i32), TryFromIntError> {
                let x: i32 = (value >> 38).try_into()?;
                let y: i16 = (value << 52 >> 52).try_into()?;
                let z: i32 = (value << 26 >> 38).try_into()?;

                Ok((x, y, z))
            }

            let (x, y, z) = calc_x_y_z(value)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

            Ok(Position { x, y, z })
        }
    }
}

mod animation {
    use byteorder::ReadBytesExt;

    use crate::packets::play::clientbound::animation::EntityAnimationType;

    use super::Decodable;

    impl Decodable for EntityAnimationType {
        fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, std::io::Error> {
            let animation_id = reader.read_u8()?;

            Ok(match animation_id {
                0x0 => Self::SwingMainArm,
                0x1 => Self::TakeDamage,
                0x2 => Self::LeaveBed,
                0x3 => Self::SwingOffHand,
                0x4 => Self::CriticalEffect,
                0x5 => Self::MagicCriticalEffect,
                _ => Err(std::io::Error::new(
                    std::io::ErrorKind::Unsupported,
                    format!("Unsupported animation ID, {}", animation_id),
                ))?,
            })
        }
    }
}

mod statistics {
    use crate::packets::play::clientbound::statistics::{CategoryType, Statistic};

    use super::Decodable;

    impl Decodable for Statistic {
        fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, std::io::Error> {
            let category_id = todo!("read varint");
            let statistic_id = todo!("read varint");
            let value = todo!("read varint");

            Ok(Statistic {
                category: match category_id {
                    0 => CategoryType::Mined,
                    1 => CategoryType::Crafted,
                    2 => CategoryType::Used,
                    3 => CategoryType::Broken,
                    4 => CategoryType::PickedUp,
                    5 => CategoryType::Dropped,
                    6 => CategoryType::Killed,
                    7 => CategoryType::KilledBy,
                    8 => CategoryType::Custom,
                    _ => panic!("Invalid Category ID"),
                },
                statistic_id,
                value,
            })
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{decoding::Decodable, packets::login::LoginRequest};
    use std::io::Cursor;

    #[test]
    fn login_request_test() {
        let data = [
            0, 1, 47, 200, 65, 125, 158, 139, 71, 11, 155, 115, 170, 161, 79, 225, 119, 188, 3, 78,
            86, 54,
        ];
        let cursor = &mut Cursor::new(&data);
        println!("{:?}, {:?}", data, LoginRequest::decode(cursor));
    }
}
