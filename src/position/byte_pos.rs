use crate::position::Position;
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BytePos(pub u32);

impl From<BytePos> for u32 {
    fn from(byte_pos: BytePos) -> Self {
        byte_pos.0
    }
}

impl From<u32> for BytePos {
    fn from(value: u32) -> Self {
        BytePos(value)
    }
}

// impl Add for BytePos {
//     type Output<'a> = &'a Self;

//     fn add(self, rhs: Self) -> &'a Self {
//         Self(self.0 + rhs.0)
//     }
// }

impl Add for BytePos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl Sub for BytePos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl Position for BytePos {
    fn with_one_added(&self) -> Self {
        *self + BytePos(1)
    }
}
