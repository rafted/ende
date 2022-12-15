use crate::{decoding::Decodable, encoding::Encodable};
use proc_macros::ParsePacket;
use std::io::{Cursor, Read};

use self::{
    login::LoginRequest,
    play::clientbound::{animation::EntityAnimation, SpawnEntity, SpawnExperienceOrb, SpawnPlayer},
    status::clientbound::{PingResponse, StatusResponse},
};

#[derive(ParsePacket)]
pub enum PacketType {
    #[packet(0x00, StatusResponse)]
    StatusResponseType,
    #[packet(0x01, PingResponse)]
    PingResponseType,
    #[packet(0x00, LoginRequest)]
    LoginRequestType,
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
                0x00 => PacketType::LoginRequestType,
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

pub mod status {
    pub mod clientbound {
        use crate::encoding::Encodable;
        use crate::{decoding::Decodable, VarInt};
        use proc_macros::MinecraftPacket;
        use std::io::{Read, Write};

        // 0x00
        #[derive(MinecraftPacket, Debug, PartialEq)]
        pub struct StatusResponse {
            pub id: VarInt,
            pub response: String,
        }

        // 0x01
        #[derive(MinecraftPacket, Debug, PartialEq)]
        pub struct PingResponse {
            pub id: VarInt,
            pub payload: i64,
        }
    }
}

pub mod login {

    use crate::encoding::Encodable;
    use crate::{decoding::Decodable, VarInt};
    use proc_macros::MinecraftPacket;
    use std::io::{Read, Write};
    use uuid::Uuid;

    #[derive(MinecraftPacket, Debug, PartialEq)]
    pub struct LoginRequest {
        pub id: VarInt,
        pub uuid: Option<Uuid>,
        pub username: String,
    }
}

pub mod play {
    pub mod clientbound {
        use crate::encoding::Encodable;
        use crate::position::Angle;
        use crate::{decoding::Decodable, VarInt};
        use proc_macros::MinecraftPacket;
        use std::io::{Read, Write};
        use uuid::Uuid;

        #[derive(MinecraftPacket, Debug, PartialEq)]
        pub struct SpawnEntity {
            pub id: VarInt,
            pub entity_id: VarInt,
            pub entity_unique_id: Uuid,
            pub ty: VarInt,
            pub x: f64,
            pub y: f64,
            pub z: f64,
            pub pitch: Angle,
            pub yaw: Angle,
            pub head_yaw: Angle,
            pub data: VarInt,
            pub velocity_x: i16,
            pub velocity_y: i16,
            pub velocity_z: i16,
        }

        #[derive(MinecraftPacket, Debug, PartialEq)]
        pub struct SpawnExperienceOrb {
            pub id: VarInt,
            pub entity_id: VarInt,
            pub x: f64,
            pub y: f64,
            pub z: f64,
            pub count: i16,
        }

        #[derive(MinecraftPacket, Debug, PartialEq)]
        pub struct SpawnPlayer {
            pub id: VarInt,
            pub entity_id: VarInt,
            pub unique_id: Uuid,
            pub x: f64,
            pub y: f64,
            pub z: f64,
            pub yaw: Angle,
            pub pitch: Angle,
        }

        pub mod animation {
            use crate::encoding::Encodable;
            use crate::{decoding::Decodable, VarInt};
            use proc_macros::MinecraftPacket;
            use std::io::{Read, Write};

            #[derive(MinecraftPacket, Debug, PartialEq)]
            pub struct EntityAnimation {
                pub id: VarInt,
                pub entity_id: VarInt,
                pub animation: EntityAnimationType,
            }

            #[derive(PartialEq, Debug)]
            pub enum EntityAnimationType {
                SwingMainArm = 0,
                TakeDamage = 1,
                LeaveBed = 2,
                SwingOffHand = 3,
                CriticalEffect = 4,
                MagicCriticalEffect = 5,
            }
        }

        pub mod statistics {
            use crate::encoding::Encodable;
            use crate::{decoding::Decodable, VarInt};
            use proc_macros::MinecraftPacket;
            use std::io::{Read, Write};

            #[derive(PartialEq, Debug)]
            pub enum CategoryType {
                Mined = 0,
                Crafted = 1,
                Used = 2,
                Broken = 3,
                PickedUp = 4,
                Dropped = 5,
                Killed = 6,
                KilledBy = 7,
                Custom = 8,
            }

            #[derive(PartialEq, Debug)]
            pub struct Statistic {
                pub category: CategoryType, // this is the id of CategoryType
                pub statistic_id: VarInt,
                pub value: VarInt,
            }

            #[derive(MinecraftPacket, Debug, PartialEq)]
            pub struct AwardStatistics {
                pub id: VarInt,
                pub count: VarInt,
                pub statistic: Vec<Statistic>,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use uuid::Uuid;

    use crate::{
        encoding::Encodable,
        packets::{get_packet_type_from_id, PacketDirection, PacketState},
    };

    use super::{login::LoginRequest, PacketType};

    #[test]
    fn packet_type() -> Result<(), std::io::Error> {
        let request = LoginRequest {
            id: 0x00,
            uuid: Some(Uuid::from_str("2fc8417d-9e8b-470b-9b73-aaa14fe177bc").unwrap()),
            username: String::from("NV6"),
        };

        let data = &mut Vec::<u8>::new();

        request.encode(data).unwrap();

        let packet_type =
            get_packet_type_from_id(0x00, PacketState::Login, PacketDirection::Clientbound)?;

        if let PacketType::LoginRequestType = packet_type {
            let packet = packet_type.wrap_packet::<LoginRequest>(data)?;

            assert_eq!(request, packet, "Packet data does not match!");
            assert_eq!(0x00, packet_type.get_id());
        }

        Ok(())
    }
}
