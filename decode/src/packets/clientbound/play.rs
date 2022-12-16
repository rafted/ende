use proc_macros::MinecraftPacket;
use uuid::Uuid;

use crate::{animation::EntityAnimationType, position::Angle, statistics::Statistic, VarInt};

// 0x00
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

// 0x01
#[derive(MinecraftPacket, Debug, PartialEq)]
pub struct SpawnExperienceOrb {
    pub id: VarInt,
    pub entity_id: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub count: i16,
}

// 0x02
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

// 0x03
#[derive(MinecraftPacket, Debug, PartialEq)]
pub struct EntityAnimation {
    pub id: VarInt,
    pub entity_id: VarInt,
    pub animation: EntityAnimationType,
}

// 0x04
#[derive(MinecraftPacket, Debug, PartialEq)]
pub struct AwardStatistics {
    pub id: VarInt,
    pub count: VarInt,
    pub statistic: Vec<Statistic>,
}
