use std::ops::Add;
pub struct Triangle<T> {
    // a is the longest, and b is longer than c.
    a: T,
    b: T,
    c: T,
}

impl<T> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>>
    where
        T: PartialOrd<f64> + PartialOrd + Copy + Add<T, Output = T>,
    {
        if sides.iter().any(|s| *s <= 0.0) {
            None
        } else {
            let mut sides = sides;
            sides.sort();
            match sides[0] + sides[1] > sides[2] {
                true => Some(Triangle {
                    a: sides[2],
                    b: sides[1],
                    c: sides[0],
                }),
                false => None,
            }
        }
    }

    pub fn is_equilateral(&self) -> bool {
        match (self.a == self.b, self.b == self.c) {
            (true, true) => true,
            (_, _) => false,
        }
    }

    pub fn is_scalene(&self) -> bool {
        match (self.a == self.b, self.b == self.c) {
            (false, false) => true,
            (_, _) => false,
        }
    }

    pub fn is_isosceles(&self) -> bool {
        match (self.a == self.b, self.b == self.c) {
            (true, false) => true,
            (false, true) => true,
            (_, _) => false,
        }
    }
}
