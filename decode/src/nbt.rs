use byteorder::{BigEndian, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

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
            value if value.starts_with('[') && value.ends_with(']') => {
                // todo: save previous progress so we don't have to loop 3 times over the same vec in the worst case scenario.
                fn parse_value_array<T>(value: &str) -> Result<Vec<T>, std::io::Error>
                where
                    T: std::str::FromStr,
                    <T as std::str::FromStr>::Err: std::fmt::Debug,
                {
                    value[1..value.len() - 1]
                        .split(",")
                        .map(|s| s.trim().parse())
                        .map(|s| {
                            s.map_err(|e| {
                                std::io::Error::new(
                                    std::io::ErrorKind::Unsupported,
                                    format!("{:?}", e),
                                )
                            })
                        })
                        .collect()
                }

                if let Ok(value) = parse_value_array::<i8>(value) {
                    Value::ByteArray(value)
                } else if let Ok(value) = parse_value_array::<i32>(value) {
                    Value::IntArray(value)
                } else if let Ok(value) = parse_value_array::<i64>(value) {
                    Value::LongArray(value)
                } else {
                    return Err(
                        std::io::Error::new(std::io::ErrorKind::Unsupported, "Unsupported operation, not able to match i8, i32 or i64 in Array NBT parsing.")
                    );
                }
            }
            value if value.parse::<i8>().is_ok() => Value::Byte(value.parse::<i8>().unwrap()),
            value if value.parse::<i16>().is_ok() => Value::Short(value.parse::<i16>().unwrap()),
            value if value.parse::<i32>().is_ok() => Value::Int(value.parse::<i32>().unwrap()),
            value if value.parse::<i64>().is_ok() => Value::Long(value.parse::<i64>().unwrap()),
            value if value.parse::<f32>().is_ok() => Value::Float(value.parse::<f32>().unwrap()),
            value if value.parse::<f64>().is_ok() => Value::Double(value.parse::<f64>().unwrap()),
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
