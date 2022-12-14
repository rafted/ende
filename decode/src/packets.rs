use std::io::Cursor;

use self::login::LoginRequest;

pub trait Packet {}

pub enum PacketState {
    Login,
    Play,
}

/// This method will merely act as a bridge between packet id -> whole packet.
///
/// It will require the packet data as an &[u8], which will be changed accordingly.
/// This function will add the packet id back in front of the data, which had been removed to retrieve
/// the packet id in the first place, however we still require this id to be serialized into the packet.
///
/// All packets should be handled through this. Manual mapping, perhaps we could change to a HashMap?
/// I'm not sure if that's faster than a match statement.
pub fn get_packet_from_id(
    id: u8,
    state: PacketState,
    data: &[u8],
) -> Result<impl Packet, std::io::Error> {
    let data = [&[id], data].concat();
    let reader = &mut Cursor::new(data);

    return match state {
        PacketState::Login => match id {
            0x0 => LoginRequest::decode(reader),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Unimplemented or invalid packet id.",
            )),
        },
        PacketState::Play => match id {
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Unimplemented or invalid packet id.",
            )),
        },
    };
}

pub mod login {
    use crate::decoding::Decodable;
    use crate::encoding::Encodable;
    use proc_macros::MinecraftPacket;
    use std::io::{Read, Write};
    use uuid::Uuid;

    use super::Packet;

    #[derive(MinecraftPacket, Debug)]
    pub struct LoginRequest {
        pub id: u8,
        pub uuid: Uuid,
        pub username: String,
    }

    impl Packet for LoginRequest {}
}
