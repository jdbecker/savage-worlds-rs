use crate::dice_result::DiceResult;

pub trait CharacterTrait {
    fn roll(&self) -> DiceResult;
    fn roll_wildcard(&self) -> DiceResult;
}
