use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::packets::{Decodable, Encodable};

#[derive(PartialEq, Debug)]
pub enum EntityAnimationType {
    SwingMainArm = 0,
    TakeDamage = 1,
    LeaveBed = 2,
    SwingOffHand = 3,
    CriticalEffect = 4,
    MagicCriticalEffect = 5,
}

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
