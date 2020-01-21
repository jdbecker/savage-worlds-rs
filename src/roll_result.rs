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
