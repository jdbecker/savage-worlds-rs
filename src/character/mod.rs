use crate::character::attributes::Attributes;
use crate::character::skills::Skills;

mod attribute_types;
mod attributes;
mod skills;

pub struct Character {
    attributes: Attributes,
    skills: Skills,
}

impl Character {}
