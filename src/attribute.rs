use crate::die::Die;

pub struct Attribute {
    die: Die,
    modifier: i8,
}

impl Attribute {
    pub fn new() -> Attribute {
        Attribute {
            die: Die::d4,
            modifier: 0,
        }
    }

    pub fn advance(self) -> Attribute {
        Attribute {
            modifier: match self.die {
                Die::d12 => self.modifier + 1,
                _ => self.modifier,
            },
            die: match self.die {
                Die::d4 => Die::d6,
                Die::d6 => Die::d8,
                Die::d8 => Die::d10,
                _ => Die::d12,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advances() {
        let mut a = Attribute::new();
        assert_eq!(a.die, Die::d4);
        assert_eq!(a.modifier, 0);
        a = a.advance();
        assert_eq!(a.die, Die::d6);
        a = a.advance();
        assert_eq!(a.die, Die::d8);
        a = a.advance();
        assert_eq!(a.die, Die::d10);
        a = a.advance();
        assert_eq!(a.die, Die::d12);
        assert_eq!(a.modifier, 0);
        a = a.advance();
        assert_eq!(a.die, Die::d12);
        assert_eq!(a.modifier, 1);
    }
}
