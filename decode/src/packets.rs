use crate::decoding::Decodable;
use proc_macros::ParsePacket;
use std::io::Read;

use self::{
    login::LoginRequest,
    status::clientbound::{PingResponse, StatusResponse},
};

#[derive(ParsePacket)]
pub enum PacketType {
    #[packet(type = StatusResponse)]
    StatusResponseType,
    #[packet(type = PingResponse)]
    PingResponseType,
    #[packet(type = LoginRequest)]
    LoginRequestType,
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
    // data: &[u8],
) -> Result<PacketType, std::io::Error> {
    // let data = [&[id], data].concat();
    // let reader = &mut Cursor::new(data);

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
            _ => invalid_state_error?,
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
        pub uuid: Uuid,
        pub username: String,
    }
}

#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::{
        encoding::Encodable,
        packets::{get_packet_type_from_id, PacketDirection, PacketState},
    };

    use super::{login::LoginRequest, PacketType};
    use std::io::Cursor;

    #[test]
    fn packet_type() {
        let request = LoginRequest {
            id: 0x00,
            uuid: Uuid::new_v4(),
            username: String::from("NV6"),
        };

        let data = &mut Vec::<u8>::new();

        request.encode(data).unwrap();

        let packet_type =
            get_packet_type_from_id(0x00, PacketState::Login, PacketDirection::Clientbound)
                .unwrap();

        let cursor = &mut Cursor::new(&data);

        if let PacketType::LoginRequestType = packet_type {
            assert_eq!(
                request,
                packet_type.parse_login_request(cursor).unwrap(),
                "Packet data does not match!"
            );
        }
    }
}
