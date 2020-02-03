use crate::die::Die;

pub struct DieResult {
    pub die: Die,
    pub modifier: u8,
    pub roll: u8,
}

impl DieResult {
    pub fn total(&self) -> u8 {
        self.modifier + self.roll
    }

    fn summarize(&self) -> String {
        let die_name = if self.modifier == 0 {
            format!("{:?}", self.die)
        } else {
            format!("{:?}+{}", self.die, self.modifier)
        };
        if self.roll > self.die.max() {
            let max_rolls: Vec<String> =
                vec![self.die.max().to_string(); (self.roll / self.die.max()) as usize];
            format!(
                "[{}:{}({},{})]",
                die_name,
                self.total(),
                max_rolls.join(","),
                self.roll % self.die.max()
            )
        } else {
            format!("[{}:{}({})]", die_name, self.total(), self.roll)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::die::Die::d4;

    #[test]
    fn simple_die_result() {
        let r = DieResult {
            die: d4,
            modifier: 0,
            roll: 3,
        };
        assert_eq!("[d4:3(3)]", r.summarize())
    }

    #[test]
    fn die_result_modifier() {
        let r = DieResult {
            die: d4,
            modifier: 2,
            roll: 2,
        };
        assert_eq!("[d4+2:4(2)]", r.summarize())
    }

    #[test]
    fn die_result_explode() {
        let r = DieResult {
            die: d4,
            modifier: 0,
            roll: 9,
        };
        assert_eq!("[d4:9(4,4,1)]", r.summarize())
    }

    #[test]
    fn die_result_modifier_explode() {
        let r = DieResult {
            die: d4,
            modifier: 3,
            roll: 7,
        };
        assert_eq!("[d4+3:10(4,3)]", r.summarize())
    }
}
