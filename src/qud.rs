use std::ops::{Add, Mul, Not};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Qud {
    Zero = 0,
    One = 1,
    Super = 2,
    Error = 3,
}

impl Qud {
    pub fn from_u8(val: u8) -> Option<Qud> {
        match val {
            0 => Some(Qud::Zero),
            1 => Some(Qud::One),
            2 => Some(Qud::Super),
            3 => Some(Qud::Error),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }

    pub fn qnot(self) -> Qud {
        match self {
            Qud::Zero => Qud::One,
            Qud::One => Qud::Super,
            Qud::Super => Qud::Error,
            Qud::Error => Qud::Zero,
        }
    }

    pub fn qmax(self, other: Qud) -> Qud {
        let vals = [self as u8, other as u8];
        Qud::from_u8(*vals.iter().max().unwrap()).unwrap()
    }

    pub fn qmin(self, other: Qud) -> Qud {
        let vals = [self as u8, other as u8];
        Qud::from_u8(*vals.iter().min().unwrap()).unwrap()
    }
}

impl Not for Qud {
    type Output = Qud;
    fn not(self) -> Qud {
        self.qnot()
    }
}

impl Add for Qud {
    type Output = Qud;
    fn add(self, other: Qud) -> Qud {
        let sum = (self as u8 + other as u8) % 4;
        Qud::from_u8(sum).unwrap()
    }
}

impl Mul for Qud {
    type Output = Qud;
    fn mul(self, other: Qud) -> Qud {
        if self == Qud::Error || other == Qud::Error {
            return Qud::Error;
        }
        Qud::from_u8((self as u8 * other as u8) % 4).unwrap()
    }
}
