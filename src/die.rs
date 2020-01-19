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
    fn max(&self) -> u8 {
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
}
