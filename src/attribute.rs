use crate::character_trait::CharacterTrait;
use crate::dice_result::DiceResult;
use crate::die::Die;

pub struct Attribute {
    die: Die,
    modifier: u8,
}

impl Attribute {
    pub fn new(die: Die) -> Attribute {
        Attribute { die, modifier: 0 }
    }
}

impl CharacterTrait for Attribute {
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
    fn roll_simple_attr() {
        let a = Attribute {
            die: Die::d4,
            modifier: 0,
        };
        assert!(Regex::new(r"^\d+ = \[d4:\d+(\([\d,]+\))?]$")
            .unwrap()
            .is_match(&a.roll().summarize_max()))
    }

    #[test]
    fn roll_attr_modifier() {
        let a = Attribute {
            die: Die::d4,
            modifier: 2,
        };
        assert!(Regex::new(r"^\d+ = 2 \+ \[d4:\d+(\([\d,]+\))?]$")
            .unwrap()
            .is_match(&a.roll().summarize_max()))
    }

    #[test]
    fn roll_wildcard() {
        let a = Attribute {
            die: Die::d4,
            modifier: 0,
        };
        assert!(
            Regex::new(r"^\d+ = \[d4:\d+(\([\d,]+\))?], \[d6:\d+(\([\d,]+\))?]$")
                .unwrap()
                .is_match(&a.roll_wildcard().summarize_max())
        )
    }

    #[test]
    fn roll_wildcard_modifier() {
        let a = Attribute {
            die: Die::d4,
            modifier: 1,
        };
        assert!(
            Regex::new(r"^\d+ = 1 \+ \[d4:\d+(\([\d,]+\))?], \[d6:\d+(\([\d,]+\))?]$")
                .unwrap()
                .is_match(&a.roll_wildcard().summarize_max())
        )
    }
}
