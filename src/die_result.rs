use crate::die::Die;

pub struct DieResult {
    pub die: Die,
    pub roll: u8,
}

impl DieResult {
    fn summarize(&self) -> String {
        if self.roll > self.die.max() {
            let max_rolls: Vec<String> =
                vec![self.die.max().to_string(); (self.roll / self.die.max()) as usize];
            format!(
                "[{:?}:{}({},{})]",
                self.die,
                self.roll,
                max_rolls.join(","),
                self.roll % self.die.max()
            )
        } else {
            format!("[{:?}:{}({})]", self.die, self.roll, self.roll)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::die::Die::d4;

    #[test]
    fn simple_die_result() {
        let r = DieResult { die: d4, roll: 3 };
        assert_eq!("[d4:3(3)]", r.summarize())
    }

    #[test]
    fn die_result_explode() {
        let r = DieResult { die: d4, roll: 9 };
        assert_eq!("[d4:9(4,4,1)]", r.summarize())
    }
}
