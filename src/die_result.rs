use crate::die::Die;

pub struct DieResult {
    pub die: Die,
    pub result: u8,
}

impl DieResult {
    pub fn summarize(&self) -> String {
        if self.result > self.die.max() {
            let max_rolls: Vec<String> =
                vec![self.die.max().to_string(); (self.result / self.die.max()) as usize];
            format!(
                "[{:?}:{}({},{})]",
                self.die,
                self.result,
                max_rolls.join(","),
                self.result % self.die.max()
            )
        } else {
            format!("[{:?}:{}]", self.die, self.result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::die::Die::d4;

    #[test]
    fn simple_die_result() {
        let r = DieResult { die: d4, result: 3 };
        assert_eq!("[d4:3]", r.summarize())
    }

    #[test]
    fn die_result_explode() {
        let r = DieResult { die: d4, result: 9 };
        assert_eq!("[d4:9(4,4,1)]", r.summarize())
    }
}
