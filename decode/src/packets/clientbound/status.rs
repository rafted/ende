use crate::VarInt;
use proc_macros::MinecraftPacket;

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
