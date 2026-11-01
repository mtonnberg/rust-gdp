//! Checked integers of arbitrary bit width from 1 through 128.
use crate::{
    predicates::{Even, InRange, NonZero, Odd, Positive},
    Predicate,
};

/// An unsigned `BITS`-bit integer stored in a `u128`.
/// Invalid widths and out-of-range values are rejected, never truncated.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct UInt<const BITS: u32>(u128);
impl<const BITS: u32> UInt<BITS> {
    /// Maximum representable value, or None for an invalid width.
    pub const MAX: Option<u128> = if BITS == 0 || BITS > 128 {
        None
    } else {
        Some(u128::MAX >> (128 - BITS))
    };
    /// Construct without truncating.
    pub const fn new(value: u128) -> Option<Self> {
        match Self::MAX {
            Some(max) if value <= max => Some(Self(value)),
            _ => None,
        }
    }
    /// Underlying value.
    pub const fn get(self) -> u128 {
        self.0
    }
    /// Addition with bit-width overflow detection.
    pub const fn checked_add(self, other: Self) -> Option<Self> {
        match self.0.checked_add(other.0) {
            Some(v) => Self::new(v),
            None => None,
        }
    }
    /// Subtraction with underflow detection.
    pub const fn checked_sub(self, other: Self) -> Option<Self> {
        match self.0.checked_sub(other.0) {
            Some(v) => Self::new(v),
            None => None,
        }
    }
    /// Multiplication with bit-width overflow detection.
    pub const fn checked_mul(self, other: Self) -> Option<Self> {
        match self.0.checked_mul(other.0) {
            Some(v) => Self::new(v),
            None => None,
        }
    }
}
/// A thirteen-bit unsigned integer (0..=8191), with the layout of `u16`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct U13(u16);
impl U13 {
    /// Maximum thirteen-bit value.
    pub const MAX: u16 = 8191;
    /// Construct without truncating.
    pub const fn new(value: u16) -> Option<Self> {
        if value <= Self::MAX {
            Some(Self(value))
        } else {
            None
        }
    }
    /// Underlying value.
    pub const fn get(self) -> u16 {
        self.0
    }
    /// Addition with thirteen-bit overflow detection.
    pub const fn checked_add(self, other: Self) -> Option<Self> {
        match self.0.checked_add(other.0) {
            Some(v) => Self::new(v),
            None => None,
        }
    }
    /// Subtraction with underflow detection.
    pub const fn checked_sub(self, other: Self) -> Option<Self> {
        match self.0.checked_sub(other.0) {
            Some(v) => Self::new(v),
            None => None,
        }
    }
    /// Multiplication with thirteen-bit overflow detection.
    pub const fn checked_mul(self, other: Self) -> Option<Self> {
        match self.0.checked_mul(other.0) {
            Some(v) => Self::new(v),
            None => None,
        }
    }
}
impl Predicate<U13> for Positive {
    fn test(v: &U13) -> bool {
        v.0 > 0
    }
}
impl Predicate<U13> for NonZero {
    fn test(v: &U13) -> bool {
        v.0 != 0
    }
}
impl Predicate<U13> for Even {
    fn test(v: &U13) -> bool {
        v.0 % 2 == 0
    }
}
impl Predicate<U13> for Odd {
    fn test(v: &U13) -> bool {
        v.0 % 2 != 0
    }
}
impl<const L: u128, const H: u128> Predicate<U13> for InRange<L, H> {
    fn test(v: &U13) -> bool {
        L <= v.0 as u128 && v.0 as u128 <= H
    }
}
impl<const B: u32> Predicate<UInt<B>> for Positive {
    fn test(v: &UInt<B>) -> bool {
        v.0 > 0
    }
}
impl<const B: u32> Predicate<UInt<B>> for NonZero {
    fn test(v: &UInt<B>) -> bool {
        v.0 != 0
    }
}
impl<const B: u32> Predicate<UInt<B>> for Even {
    fn test(v: &UInt<B>) -> bool {
        v.0 % 2 == 0
    }
}
impl<const B: u32> Predicate<UInt<B>> for Odd {
    fn test(v: &UInt<B>) -> bool {
        v.0 % 2 != 0
    }
}
impl<const B: u32, const L: u128, const H: u128> Predicate<UInt<B>> for InRange<L, H> {
    fn test(v: &UInt<B>) -> bool {
        L <= v.0 && v.0 <= H
    }
}
