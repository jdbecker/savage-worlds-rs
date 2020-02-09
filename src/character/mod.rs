use crate::character::attributes::Attributes;
use crate::character::skills::Skills;

mod attribute;
mod attribute_type;
mod attributes;
mod character_trait;
mod rank;
mod skill;
mod skills;

pub struct Character {
    attributes: Attributes,
    skills: Skills,
}

impl Character {}
