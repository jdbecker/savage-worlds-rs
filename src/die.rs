use rand::Rng;

#[derive(PartialEq, Debug)]
pub enum Die {
    d4,
    d6,
    d8,
    d10,
    d12,
}

impl Die {
    pub fn max(&self) -> u8 {
        match self {
            Die::d4 => 4,
            Die::d6 => 6,
            Die::d8 => 8,
            Die::d10 => 10,
            Die::d12 => 12,
        }
    }

    pub fn roll(&self) -> u8 {
        rand::thread_rng().gen_range(1, self.max() + 1)
    }

    pub fn roll_explode(&self) -> u8 {
        let mut total = 0;
        let mut roll = self.max();
        while roll == self.max() {
            roll = self.roll();
            total += roll;
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roll(d: &Die) -> u8 {
        let r = d.roll();
        assert!(r >= 1, "die roll was {}", r);
        assert!(r <= d.max(), "die roll was {}", r);
        r
    }

    fn roll_until_explode(d: &Die) -> u8 {
        let mut r = 0;
        while r < d.max() {
            r = d.roll_explode();
            assert!(r >= 1, "die roll was {}", r);
            assert_ne!(
                r % d.max(),
                0,
                "exploding die result shouldn't be divisible by max"
            );
        }
        r
    }

    #[test]
    fn can_roll_1_on_dx() {
        let d = Die::d4;
        let mut r = roll(&d);
        while r != 1 {
            r = roll(&d);
        }
    }

    #[test]
    fn can_roll_4_on_d4() {
        let d = Die::d4;
        let mut r = roll(&d);
        while r != 4 {
            r = roll(&d);
        }
    }

    #[test]
    fn can_roll_6_on_d6() {
        let d = Die::d6;
        let mut r = roll(&d);
        while r != 6 {
            r = roll(&d);
        }
    }

    #[test]
    fn can_roll_8_on_d8() {
        let d = Die::d8;
        let mut r = roll(&d);
        while r != 8 {
            r = roll(&d);
        }
    }

    #[test]
    fn can_roll_10_on_d10() {
        let d = Die::d10;
        let mut r = roll(&d);
        while r != 10 {
            r = roll(&d);
        }
    }

    #[test]
    fn can_roll_12_on_d12() {
        let d = Die::d12;
        let mut r = roll(&d);
        while r != 12 {
            r = roll(&d);
        }
    }

    #[test]
    fn explode_mod_not_zero() {
        let d = Die::d4;
        let r = roll_until_explode(&d);
        assert!(r > 4, "die roll was {}", r)
    }

    #[test]
    fn roll_explode_not_always() {
        let d = Die::d12;
        let mut r = 12;
        while r >= 12 {
            r = d.roll_explode();
        }
        assert!(r >= 1, "die roll was {}", r);
        assert!(r < 12, "die roll was {}", r);
    }
}
