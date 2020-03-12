use std::collections::HashMap;

use crate::character::attribute::Attribute;
use crate::character::attribute_type::AttributeType;
use crate::character::rank::Rank;
use crate::character::skill::Skill;

mod attribute;
mod attribute_type;
mod character_trait;
mod rank;
mod skill;

pub struct Character {
    pub rank: Rank,
    pub attributes: HashMap<AttributeType, Attribute>,
    pub skills: HashMap<String, Skill>,
}

impl Default for Character {
    fn default() -> Self {
        let mut skills = HashMap::new();
        skills.insert(
            "Athletics".to_string(),
            Skill::trained(AttributeType::Agility),
        );
        skills.insert(
            "Common Knowledge".to_string(),
            Skill::trained(AttributeType::Smarts),
        );
        skills.insert("Notice".to_string(), Skill::trained(AttributeType::Smarts));
        skills.insert(
            "Persuasion".to_string(),
            Skill::trained(AttributeType::Spirit),
        );
        skills.insert(
            "Stealth".to_string(),
            Skill::trained(AttributeType::Agility),
        );
        let mut attributes = HashMap::new();
        attributes.insert(AttributeType::Agility, Attribute::default());
        attributes.insert(AttributeType::Smarts, Attribute::default());
        attributes.insert(AttributeType::Spirit, Attribute::default());
        attributes.insert(AttributeType::Strength, Attribute::default());
        attributes.insert(AttributeType::Vigor, Attribute::default());
        Character {
            rank: Rank::Novice,
            attributes,
            skills,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::character::character_trait::CharacterTrait;
    use crate::character::rank::Rank;
    use crate::character::Character;

    #[test]
    fn create_char() {
        let c = Character::default();
        assert_eq!(Rank::Novice, c.rank);
    }

    #[test]
    fn roll_athletics() {
        let c = Character::default();
        c.skills
            .get("Athletics")
            .expect("Should have Athletics, it's a core skill!")
            .roll();
    }
}
