use crate::die_result::DieResult;

pub struct DiceResult {
    pub results: Vec<DieResult>,
    pub modifier: u8,
}

impl DiceResult {
    pub fn total(&self) -> u8 {
        self.results.iter().map(|d| d.result).sum::<u8>() + self.modifier
    }

    pub fn summarize_total(&self) -> String {
        format!("{} = {}", self.total(), self.summarize_right(" + "))
    }

    pub fn max(&self) -> u8 {
        self.results.iter().map(|d| d.result).max().unwrap_or(0) + self.modifier
    }

    pub fn summarize_max(&self) -> String {
        format!("{} = {}", self.max(), self.summarize_right(", "))
    }

    fn summarize_right(&self, join_str: &str) -> String {
        if self.modifier == 0 {
            self.summarize(join_str)
        } else {
            format!("{} + {}", self.modifier, self.summarize(join_str))
        }
    }

    fn summarize(&self, join_str: &str) -> String {
        // 8 = [d4:6(4+2)], [d6:5]
        self.results
            .iter()
            .map(|d| d.summarize())
            .collect::<Vec<String>>()
            .join(join_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::die::Die;

    #[test]
    fn simple_total() {
        let d1 = DieResult {
            die: Die::d4,
            result: 3,
        };
        let r = DiceResult {
            results: vec![d1],
            modifier: 0,
        };
        assert_eq!(r.total(), 3);
    }

    #[test]
    fn multi_total() {
        let d1 = DieResult {
            die: Die::d4,
            result: 3,
        };
        let d2 = DieResult {
            die: Die::d6,
            result: 4,
        };
        let r = DiceResult {
            results: vec![d1, d2],
            modifier: 0,
        };
        assert_eq!(r.total(), 7)
    }

    #[test]
    fn multi_total_modifier() {
        let d1 = DieResult {
            die: Die::d4,
            result: 2,
        };
        let d2 = DieResult {
            die: Die::d6,
            result: 5,
        };
        let r = DiceResult {
            results: vec![d1, d2],
            modifier: 1,
        };
        assert_eq!(r.total(), 8);
    }

    #[test]
    fn multi_max() {
        let d1 = DieResult {
            die: Die::d4,
            result: 4,
        };
        let d2 = DieResult {
            die: Die::d6,
            result: 2,
        };
        let r = DiceResult {
            results: vec![d1, d2],
            modifier: 0,
        };
        assert_eq!(r.max(), 4);
    }

    #[test]
    fn multi_max_modifier() {
        let d1 = DieResult {
            die: Die::d4,
            result: 3,
        };
        let d2 = DieResult {
            die: Die::d6,
            result: 6,
        };
        let r = DiceResult {
            results: vec![d1, d2],
            modifier: 2,
        };
        assert_eq!(r.max(), 8);
    }

    #[test]
    fn display_total_no_modifier() {
        let d1 = DieResult {
            die: Die::d4,
            result: 3,
        };
        let d2 = DieResult {
            die: Die::d6,
            result: 7,
        };
        let r = DiceResult {
            results: vec![d1, d2],
            modifier: 0,
        };
        assert_eq!(r.summarize_total(), "10 = [d4:3] + [d6:7(6,1)]")
    }

    #[test]
    fn display_total_modifier() {
        let d1 = DieResult {
            die: Die::d4,
            result: 9,
        };
        let d2 = DieResult {
            die: Die::d6,
            result: 8,
        };
        let r = DiceResult {
            results: vec![d1, d2],
            modifier: 2,
        };
        assert_eq!(r.summarize_total(), "19 = 2 + [d4:9(4,4,1)] + [d6:8(6,2)]")
    }

    #[test]
    fn display_max_no_modifier() {
        let d1 = DieResult {
            die: Die::d4,
            result: 9,
        };
        let d2 = DieResult {
            die: Die::d6,
            result: 4,
        };
        let r = DiceResult {
            results: vec![d1, d2],
            modifier: 0,
        };
        assert_eq!(r.summarize_max(), "9 = [d4:9(4,4,1)], [d6:4]")
    }

    #[test]
    fn display_max_modifier() {
        let d1 = DieResult {
            die: Die::d4,
            result: 7,
        };
        let d2 = DieResult {
            die: Die::d6,
            result: 8,
        };
        let r = DiceResult {
            results: vec![d1, d2],
            modifier: 3,
        };
        assert_eq!(r.summarize_max(), "11 = 3 + [d4:7(4,3)], [d6:8(6,2)]")
    }
}
