use rand::Rng;

pub struct Die {
    min: u8,
    max: u8,
}

impl Die {
    pub fn new(min: u8, max: u8) -> Die {
        Die { min, max }
    }

    pub fn dx(max: u8) -> Die {
        Die::new(1, max)
    }

    pub fn d4() -> Die {
        Die::dx(4)
    }

    pub fn d6() -> Die {
        Die::dx(6)
    }

    pub fn roll(&self) -> u8 {
        rand::thread_rng().gen_range(self.min, self.max + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_roll_dx() {
        let d = Die::dx(2);
        let r = d.roll();
        assert!(r >= 1, "die roll was {}", r);
        assert!(r <= 2, "die roll was {}", r);
    }

    #[test]
    fn can_roll_d4() {
        let d = Die::d4();
        let r = d.roll();
        assert!(r >= 1, "die roll was {}", r);
        assert!(r <= 4, "die roll was {}", r);
    }

    #[test]
    fn can_roll_d6() {
        let d = Die::d6();
        let r = d.roll();
        assert!(r >= 1, "die roll was {}", r);
        assert!(r <= 6, "die roll was {}", r);
    }
}
