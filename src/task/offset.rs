use std::cmp::Ordering;

#[derive(Debug)]
pub enum Offset {
    Pos(usize),
    Neg(usize),
}

impl Offset {
    pub fn apply(&self, num: usize) -> usize {
        match self {
            Offset::Pos(v) => num.checked_add(*v).unwrap(),
            Offset::Neg(v) => num.checked_sub(*v).unwrap(),
        }
    }

    pub fn add(a: Offset, b: Offset) -> Self {
        match (a, b) {
            (Offset::Pos(a), Offset::Pos(b)) => Offset::Pos(a + b),
            (Offset::Pos(a), Offset::Neg(b)) => {
                if b > a {
                    Offset::Neg(b - a)
                } else {
                    Offset::Pos(a - b)
                }
            }
            (Offset::Neg(a), Offset::Pos(b)) => {
                if a > b {
                    Offset::Neg(a - b)
                } else {
                    Offset::Pos(b - a)
                }
            }
            (Offset::Neg(a), Offset::Neg(b)) => Offset::Neg(a + b),
        }
    }

    pub fn from_diff(a: usize, b: usize) -> Self {
        let abs_diff = a.abs_diff(b);
        match a.cmp(&b) {
            Ordering::Less => Offset::Neg(abs_diff),
            Ordering::Equal => Offset::Pos(0),
            Ordering::Greater => Offset::Pos(abs_diff),
        }
    }
}
