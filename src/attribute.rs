use crate::dice_result::DiceResult;
use crate::die::Die;

pub struct Attribute {
    die: Die,
    modifier: i8,
}

impl Attribute {
    pub fn new() -> Attribute {
        Attribute {
            die: Die::d4,
            modifier: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
