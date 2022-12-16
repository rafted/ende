pub mod animation;
pub mod datatypes;
pub mod decoding;
pub mod encoding;
pub mod nbt;
pub mod packets;
pub mod position;
pub mod statistics;

#[derive(Debug, Clone, PartialEq)]
pub struct VarInt(i32);
