use crate::{
    packets::{Decodable, Encodable},
    VarInt,
};

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

impl Decodable for Statistic {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, std::io::Error> {
        let category_id = todo!("read varint");
        let statistic_id = todo!("read varint");
        let value = todo!("read varint");

        Ok(Statistic {
            category: match category_id {
                0 => CategoryType::Mined,
                1 => CategoryType::Crafted,
                2 => CategoryType::Used,
                3 => CategoryType::Broken,
                4 => CategoryType::PickedUp,
                5 => CategoryType::Dropped,
                6 => CategoryType::Killed,
                7 => CategoryType::KilledBy,
                8 => CategoryType::Custom,
                _ => panic!("Invalid Category ID"),
            },
            statistic_id,
            value,
        })
    }
}

impl Encodable for Statistic {
    fn encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        let category_id = match self.category {
            CategoryType::Mined => 0,
            CategoryType::Crafted => 1,
            CategoryType::Used => 2,
            CategoryType::Broken => 3,
            CategoryType::PickedUp => 4,
            CategoryType::Dropped => 5,
            CategoryType::Killed => 6,
            CategoryType::KilledBy => 7,
            CategoryType::Custom => 8,
        };

        let statistic_id = self.statistic_id;
        let value = self.value;

        todo!("write varint (category_id)");
        todo!("write varint (statistic_id)");
        todo!("write varint (value)");
    }
}
