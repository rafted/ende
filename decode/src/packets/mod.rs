pub mod clientbound;
pub mod serverbound;

use std::io::{Read, Write, Cursor};

use proc_macros::ParsePacket;

use self::clientbound::{play::*, status::*};
// use self::clientbound::{login::*, play::*, status::*};
// use self::serverbound::{login::*, play::*, status::*};

pub trait Decodable: Sized {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, std::io::Error>;
}

pub trait Encodable {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error>;
}

#[derive(ParsePacket)]
pub enum PacketType {
    #[packet(0x00, StatusResponse)]
    StatusResponseType,
    #[packet(0x01, PingResponse)]
    PingResponseType,
    #[packet(0x00, SpawnEntity)]
    SpawnEntityType,
    #[packet(0x01, SpawnExperienceOrb)]
    SpawnExperienceOrbType,
    #[packet(0x02, SpawnPlayer)]
    SpawnPlayerType,
    #[packet(0x03, EntityAnimation)]
    EntityAnimationType,
    #[packet(0x04, AwardStatistics)]
    AwardStatisticsType,
}

impl PacketType {
    pub fn wrap_packet<T>(&self, data: &[u8]) -> Result<T, std::io::Error>
    where
        T: Encodable + Decodable,
    {
        let id = self.get_id();
        let data = if data[0] != id {
            [&[id], data].concat()
        } else {
            [data].concat()
        };

        let reader = &mut Cursor::new(data);

        Ok(T::decode(reader)?)
    }
}

pub enum PacketState {
    Status,
    Handshake,
    Login,
    Play,
}

pub enum PacketDirection {
    Clientbound,
    Serverbound,
}

/// This method will merely act as a bridge between packet id -> whole packet.
///
/// It will require the packet data as an &[u8], which will be changed accordingly.
/// This function will add the packet id back in front of the data, which had been removed to retrieve
/// the packet id in the first place, however we still require this id to be serialized into the packet.
///
/// All packets should be handled through this. Manual mapping, perhaps we could change to a HashMap?
/// I'm not sure if that's faster than a match statement.
pub fn get_packet_type_from_id(
    id: u8,
    state: PacketState,
    direction: PacketDirection,
) -> Result<PacketType, std::io::Error> {
    let error_msg = "Unimplemented or invalid packet id.";
    let invalid_state_error = Err(std::io::Error::new(std::io::ErrorKind::NotFound, error_msg));

    return Ok(match direction {
        PacketDirection::Serverbound => match state {
            PacketState::Handshake => match id {
                0x00 => invalid_state_error?,
                _ => invalid_state_error?,
            },
            _ => invalid_state_error?,
        },
        PacketDirection::Clientbound => match state {
            PacketState::Handshake => invalid_state_error?,
            PacketState::Status => match id {
                0x00 => PacketType::StatusResponseType,
                0x01 => PacketType::PingResponseType,
                _ => invalid_state_error?,
            },
            PacketState::Login => match id {
                // 0x00 => PacketType::LoginRequestType,
                _ => invalid_state_error?,
            },
            PacketState::Play => match id {
                0x00 => PacketType::SpawnEntityType,
                0x01 => PacketType::SpawnExperienceOrbType,
                0x02 => PacketType::SpawnPlayerType,
                0x03 => PacketType::EntityAnimationType,
                0x04 => PacketType::AwardStatisticsType,
                _ => invalid_state_error?,
            },
        },
    });
}
