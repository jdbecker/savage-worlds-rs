use crate::die::Die;

pub struct RollResult {
    pub dice: Vec<(Die, u8)>,
}

impl RollResult {
    pub fn total(&self) -> u8 {
        self.dice.iter().map(|d| d.1).sum()
    }

    pub fn summarize_total(&self) -> String {
        format!("{} = {}", self.total(), self.summarize())
    }

    pub fn max(&self) -> u8 {
        self.dice.iter().map(|d| d.1).max().unwrap_or(0)
    }

    pub fn summarize_max(&self) -> String {
        format!("{} = {}", self.max(), self.summarize())
    }

    fn summarize(&self) -> String {
        // 8 = [d4:6(4+2)], [d6:5]
        self.dice
            .iter()
            .map(|d| RollResult::display_die_result(&d.0, d.1))
            .collect::<Vec<String>>()
            .join(", ")
    }

    fn display_die_result(die: &Die, result: u8) -> String {
        if result > die.max() {
            let max_rolls: Vec<String> = vec![die.max().to_string(); (result / die.max()) as usize];
            format!(
                "[{:?}:{}({},{})]",
                die,
                result,
                max_rolls.join(","),
                result % die.max()
            )
        } else {
            format!("[{:?}:{}]", die, result)
        }
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

    #[test]
    fn die_total_no_explode() {
        let r = RollResult {
            dice: vec![(d4, 3), (d6, 5)],
        };
        assert_eq!(r.summarize_total(), "8 = [d4:3], [d6:5]")
    }

    #[test]
    fn die_max_with_explode() {
        let r = RollResult {
            dice: vec![(d4, 9), (d6, 7)],
        };
        assert_eq!(r.summarize_max(), "9 = [d4:9(4,4,1)], [d6:7(6,1)]")
    }
}
