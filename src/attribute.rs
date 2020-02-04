use crate::dice_result::DiceResult;
use crate::die::Die;

pub struct Attribute {
    die: Die,
    modifier: u8,
}

impl Attribute {
    pub fn roll(&self) -> DiceResult {
        DiceResult {
            results: vec![self.die.roll_explode()],
            modifier: self.modifier,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

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
}
