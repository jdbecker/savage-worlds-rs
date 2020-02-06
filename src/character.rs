use crate::attribute::Attribute;
use crate::die::Die;

pub struct Character {
    agility: Attribute,
    smarts: Attribute,
    spirit: Attribute,
    strength: Attribute,
    vigor: Attribute,
}

impl Character {
    pub fn new() -> Character {
        Character {
            agility: Attribute::new(Die::d4),
            smarts: Attribute::new(Die::d4),
            spirit: Attribute::new(Die::d4),
            strength: Attribute::new(Die::d4),
            vigor: Attribute::new(Die::d4),
        }
    }
}
