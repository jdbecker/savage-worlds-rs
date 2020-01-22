use crate::die::Die;

pub struct RollResult {
    pub dice: Vec<(Die, u8)>,
}

impl RollResult {
    pub fn total(&self) -> u8 {
        self.dice.iter().map(|d| d.1).sum()
    }

    pub fn max(&self) -> u8 {
        self.dice.iter().map(|d| d.1).max().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::die::Die::{d4, d6};

    #[test]
    fn calc_total() {
        let r = RollResult {
            dice: vec![(d4, 3), (d6, 5)],
        };
        assert_eq!(r.total(), 8);
    }

    #[test]
    fn calc_max() {
        let r = RollResult {
            dice: vec![(d4, 3), (d6, 5)],
        };
        assert_eq!(r.max(), 5);
    }
}
