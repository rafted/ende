use crate::{decoding::Decodable, encoding::Encodable, VarInt};
use bincode::{deserialize, serialize};
use byteorder::{BigEndian, ReadBytesExt};
use proc_macros::{NBTDecoder, NBTEncoder};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

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
    Compound(HashMap<String, Value>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

fn read_string<R: std::io::Read>(read: &mut R) -> Result<String, std::io::Error>
where
    R: Copy,
{
    let len = read.read_u16::<BigEndian>()?;
    let mut buf = vec![0; len as usize];

    read.read_exact(&mut buf)?;
    String::from_utf8(buf).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

fn parse_nbt<R: std::io::Read>(read: &mut R) -> Result<Value, std::io::Error>
where
    R: Copy,
{
    // we want to skip the first byte.
    read.bytes().next().transpose()?;

    let name = read_string(read)?;
    parse_compound(read)
}

fn parse_compound<R: std::io::Read>(read: &mut R) -> Result<Value, std::io::Error>
where
    R: Copy,
{
    let mut compound = HashMap::<String, Value>::new();

    let mut reader = BufReader::new(read);
    let mut line = String::new();

    reader.read_line(&mut line)?;

    assert_eq!(
        line, "{",
        "NBT is not properly formatted. Expected opening brace."
    );

    while line != "}" {
        line.clear();
        reader.read_line(&mut line)?;

        let mut parts = line.splitn(2, ':');
        let key = parts.next().unwrap().trim().to_owned();
        let value = parts.next().unwrap().trim();

        let value = match value {
            value if value.starts_with('"') && value.ends_with('"') => {
                Value::String(String::from(&value[1..value.len() - 1]))
            }
            value if value.chars().all(|c| c.is_digit(10)) => Value::Int(
                value
                    .parse::<i32>()
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?,
            ),
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Unsupported,
                    "Unsupported text found.",
                ))
            }
        };

        compound.insert(key, value);
    }

    todo!()
}
