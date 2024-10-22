// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SaturatingU16 {
    value: u16,
}

impl Into<SaturatingU16> for &u8 {
    fn into(self) -> SaturatingU16 {
        SaturatingU16 {
            value: (*self as u16),
        }
    }
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16 { value }
    }
}

impl Into<SaturatingU16> for u8 {
    fn into(self) -> SaturatingU16 {
        SaturatingU16 {
            value: (self as u16),
        }
    }
}

impl Into<u16> for SaturatingU16 {
    fn into(self) -> u16 {
        todo!()
    }
}

impl Into<SaturatingU16> for &u16 {
    fn into(self) -> SaturatingU16 {
        SaturatingU16 {
            value: (*self as u16),
        }
    }
}

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let r: u16 = self.value.saturating_add(rhs.value);
        SaturatingU16 { value: r }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = u16;

    fn add(self, rhs: u16) -> Self::Output {
        let r: u16 = self.value.saturating_add(rhs);
        r
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        let r: u16 = self.value.saturating_add(rhs.value);
        SaturatingU16 { value: r }
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}
