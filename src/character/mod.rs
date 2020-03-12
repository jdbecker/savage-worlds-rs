use crate::character::attribute::Attribute;
use crate::character::rank::Rank;
use crate::character::skills::Skills;

mod attribute;
mod attribute_type;
mod character_trait;
mod rank;
mod skill;
mod skills;

pub struct Character {
    pub rank: Rank,
    pub agility: Attribute,
    pub smarts: Attribute,
    pub spirit: Attribute,
    pub strength: Attribute,
    pub vigor: Attribute,
    pub skills: Skills,
}

impl Character {}
