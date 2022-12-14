use crate::{decoding::Decodable, encoding::Encodable, VarInt};
use bincode::{deserialize, serialize};
use proc_macros::{NBTDecoder, NBTEncoder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, NBTEncoder, NBTDecoder)]
pub struct CompoundTag {
    pub data: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Value {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<Value>),
    Compound(CompoundTag),
}
