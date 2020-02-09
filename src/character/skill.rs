use crate::character::attribute_type::AttributeType;
use crate::character::character_trait::CharacterTrait;
use crate::dice_result::DiceResult;
use crate::die::Die;

pub struct Skill {
    pub attribute: AttributeType,
    pub die: Die,
    pub modifier: i8,
}

impl Skill {
    pub fn trained(attribute: AttributeType) -> Skill {
        Skill {
            attribute,
            die: Die::d4,
            modifier: 0,
        }
    }

    pub fn untrained(attribute: AttributeType) -> Skill {
        Skill {
            attribute,
            die: Die::d4,
            modifier: -2,
        }
    }
}

impl CharacterTrait for Skill {
    fn roll(&self) -> DiceResult {
        DiceResult {
            results: vec![self.die.roll_explode()],
            modifier: self.modifier,
        }
    }

    fn roll_wildcard(&self) -> DiceResult {
        DiceResult {
            results: vec![self.die.roll_explode(), Die::d6.roll_explode()],
            modifier: self.modifier,
        }
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    #[test]
    fn roll_untrained_skill() {
        let s = Skill::untrained(AttributeType::Agility);
        assert!(Regex::new(r"^-?\d+ = -2 \+ \[d4:\d+(\([\d,]+\))?]$")
            .unwrap()
            .is_match(&s.roll().summarize_max()))
    }

    #[test]
    fn roll_wildcard_trained() {
        let s = Skill::trained(AttributeType::Agility);
        assert!(
            Regex::new(r"^\d+ = \[d4:\d+(\([\d,]+\))?], \[d6:\d+(\([\d,]+\))?]$")
                .unwrap()
                .is_match(&s.roll_wildcard().summarize_max())
        )
    }
}
