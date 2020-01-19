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

    pub fn d8() -> Die {
        Die::dx(8)
    }

    pub fn d10() -> Die {
        Die::dx(10)
    }

    pub fn d12() -> Die {
        Die::dx(12)
    }

    pub fn roll(&self) -> u8 {
        rand::thread_rng().gen_range(self.min, self.max + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roll(d: &Die) -> u8 {
        let r = d.roll();
        assert!(r >= d.min, "die roll was {}", r);
        assert!(r <= d.max, "die roll was {}", r);
        r
    }

    #[test]
    fn can_roll_1_on_dx() {
        let d = Die::dx(2);
        let mut r = roll(&d);
        while r != 1 {
            r = roll(&d);
        }
    }

    #[test]
    fn can_roll_4_on_d4() {
        let d = Die::d4();
        let mut r = roll(&d);
        while r != 4 {
            r = roll(&d);
        }
    }

    #[test]
    fn can_roll_6_on_d6() {
        let d = Die::d6();
        let mut r = roll(&d);
        while r != 6 {
            r = roll(&d);
        }
    }

    #[test]
    fn can_roll_8_on_d8() {
        let d = Die::d8();
        let mut r = roll(&d);
        while r != 8 {
            r = roll(&d);
        }
    }

    #[test]
    fn can_roll_10_on_d10() {
        let d = Die::d10();
        let mut r = roll(&d);
        while r != 10 {
            r = roll(&d);
        }
    }

    #[test]
    fn can_roll_12_on_d12() {
        let d = Die::d12();
        let mut r = roll(&d);
        while r != 12 {
            r = roll(&d);
        }
    }
}
