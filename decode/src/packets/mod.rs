pub mod clientbound;
pub mod serverbound;

use std::io::{Cursor, Read, Write};

use proc_macros::ParsePacket;
use types::packet::{ClientState, PacketDirection};

// not actually unused imports, we use them in the macro
#[allow(unused_imports)]
use self::clientbound::{login::*, play::*, status::*};
#[allow(unused_imports)]
use self::clientbound::{play::*, status::*};
#[allow(unused_imports)]
use self::serverbound::{login::*, play::*, status::*};

pub trait Decodable: Sized {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, std::io::Error>;
}

pub trait Encodable {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error>;
}

#[derive(ParsePacket)]
pub enum PacketType {
    #[packet(0x00, Clientbound, Status, StatusResponse)]
    StatusResponseType,
    #[packet(0x01, Clientbound, Status, PingResponse)]
    PingResponseType,
    #[packet(0x00, Clientbound, Play, SpawnEntity)]
    SpawnEntityType,
    #[packet(0x01, Clientbound, Play, SpawnExperienceOrb)]
    SpawnExperienceOrbType,
    #[packet(0x02, Clientbound, Play, SpawnPlayer)]
    SpawnPlayerType,
    #[packet(0x03, Clientbound, Play, EntityAnimation)]
    EntityAnimationType,
    #[packet(0x04, Clientbound, Play, AwardStatistics)]
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
