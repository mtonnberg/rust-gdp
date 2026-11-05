//! Rust-focused, composable specifications. Text lengths count UTF-8 bytes.
use crate::Predicate;
use core::marker::PhantomData;

/// Has at least one element or byte.
pub enum NonEmpty {}
/// Contains only ASCII characters/bytes.
pub enum Ascii {}
/// Valid UTF-8 bytes.
pub enum Utf8 {}
/// No leading or trailing Unicode whitespace.
pub enum Trimmed {}
/// Has exactly `N` elements/bytes.
pub struct Length<const N: usize>;
/// Has at least `N` elements/bytes.
pub struct MinLength<const N: usize>;
/// Has at most `N` elements/bytes.
pub struct MaxLength<const N: usize>;
/// A numeric value greater than zero (NaN is rejected).
pub enum Positive {}
/// A numeric value unequal to zero (NaN is rejected).
pub enum NonZero {}
/// A finite floating-point value.
pub enum Finite {}
/// An even integer.
pub enum Even {}
/// An odd integer.
pub enum Odd {}
/// An inclusive nonnegative integer interval; reversed bounds reject all values.
pub struct InRange<const LOW: u128, const HIGH: u128>;
/// An inhabited Option.
pub enum IsSome {}
/// An empty Option.
pub enum IsNone {}
/// A successful Result.
pub enum IsOk {}
/// An error Result.
pub enum IsErr {}
/// Conjunction of two specifications (short-circuited).
pub struct All<P, Q>(PhantomData<fn() -> (P, Q)>);
/// Disjunction of two specifications (short-circuited).
pub struct Any<P, Q>(PhantomData<fn() -> (P, Q)>);
/// Negation of a decidable specification.
pub struct Not<P>(PhantomData<fn() -> P>);
/// Every element satisfies `P`; true on empty collections.
pub struct Each<P>(PhantomData<fn() -> P>);
/// Some element satisfies `P`; false on empty collections.
pub struct Exists<P>(PhantomData<fn() -> P>);
/// Elements are in nondecreasing order; incomparable values reject the predicate.
pub enum Sorted {}

impl<T: ?Sized, P: Predicate<T>> Predicate<T> for Not<P> {
    fn test(v: &T) -> bool {
        !P::test(v)
    }
}
impl<T: ?Sized, P: Predicate<T>, Q: Predicate<T>> Predicate<T> for All<P, Q> {
    fn test(v: &T) -> bool {
        P::test(v) && Q::test(v)
    }
}
impl<T: ?Sized, P: Predicate<T>, Q: Predicate<T>> Predicate<T> for Any<P, Q> {
    fn test(v: &T) -> bool {
        P::test(v) || Q::test(v)
    }
}
// Forward concrete specifications through shared references; avoid a blanket
// implementation, which would overlap the compositional predicates.
macro_rules! borrowed {
    ($($p:ty),*) => {$(impl<T: ?Sized> Predicate<&T> for $p where Self: Predicate<T> {
        fn test(v: &&T) -> bool { <Self as Predicate<T>>::test(v) }
    })*};
}
borrowed!(
    NonEmpty, Ascii, Utf8, Trimmed, Positive, NonZero, Finite, Even, Odd, IsSome, IsNone, IsOk,
    IsErr, Sorted
);
macro_rules! borrowed_const {
    ($($p:ident),*) => {$(impl<T: ?Sized, const N: usize> Predicate<&T> for $p<N> where Self: Predicate<T> {
        fn test(v: &&T) -> bool { <Self as Predicate<T>>::test(v) }
    })*};
}
borrowed_const!(Length, MinLength, MaxLength);
impl<T: ?Sized, const L: u128, const H: u128> Predicate<&T> for InRange<L, H>
where
    Self: Predicate<T>,
{
    fn test(v: &&T) -> bool {
        <Self as Predicate<T>>::test(v)
    }
}
impl<T: ?Sized, P> Predicate<&T> for Each<P>
where
    Self: Predicate<T>,
{
    fn test(v: &&T) -> bool {
        <Self as Predicate<T>>::test(v)
    }
}
impl<T: ?Sized, P> Predicate<&T> for Exists<P>
where
    Self: Predicate<T>,
{
    fn test(v: &&T) -> bool {
        <Self as Predicate<T>>::test(v)
    }
}
macro_rules! lengths {
    ($ty:ty $(, $gen:ident)*) => {
        impl<$($gen,)*> Predicate<$ty> for NonEmpty { fn test(v: &$ty) -> bool { !v.is_empty() } }
        impl<$($gen,)* const N: usize> Predicate<$ty> for Length<N> { fn test(v: &$ty) -> bool { v.len() == N } }
        impl<$($gen,)* const N: usize> Predicate<$ty> for MinLength<N> { fn test(v: &$ty) -> bool { v.len() >= N } }
        impl<$($gen,)* const N: usize> Predicate<$ty> for MaxLength<N> { fn test(v: &$ty) -> bool { v.len() <= N } }
    }
}
lengths!(str);
lengths!([T], T);
#[cfg(feature = "alloc")]
lengths!(alloc::string::String);
#[cfg(feature = "alloc")]
lengths!(alloc::vec::Vec<T>, T);
impl Predicate<str> for Ascii {
    fn test(v: &str) -> bool {
        v.is_ascii()
    }
}
impl Predicate<[u8]> for Ascii {
    fn test(v: &[u8]) -> bool {
        v.is_ascii()
    }
}
impl Predicate<[u8]> for Utf8 {
    fn test(v: &[u8]) -> bool {
        core::str::from_utf8(v).is_ok()
    }
}
impl Predicate<str> for Trimmed {
    fn test(v: &str) -> bool {
        v.trim() == v
    }
}
#[cfg(feature = "alloc")]
impl Predicate<alloc::string::String> for Ascii {
    fn test(v: &alloc::string::String) -> bool {
        v.is_ascii()
    }
}
#[cfg(feature = "alloc")]
impl Predicate<alloc::string::String> for Trimmed {
    fn test(v: &alloc::string::String) -> bool {
        v.trim() == v
    }
}
impl<T> Predicate<Option<T>> for IsSome {
    fn test(v: &Option<T>) -> bool {
        v.is_some()
    }
}
impl<T> Predicate<Option<T>> for IsNone {
    fn test(v: &Option<T>) -> bool {
        v.is_none()
    }
}
impl<T, E> Predicate<Result<T, E>> for IsOk {
    fn test(v: &Result<T, E>) -> bool {
        v.is_ok()
    }
}
impl<T, E> Predicate<Result<T, E>> for IsErr {
    fn test(v: &Result<T, E>) -> bool {
        v.is_err()
    }
}
impl<T, P: Predicate<T>> Predicate<[T]> for Each<P> {
    fn test(v: &[T]) -> bool {
        v.iter().all(P::test)
    }
}
impl<T, P: Predicate<T>> Predicate<[T]> for Exists<P> {
    fn test(v: &[T]) -> bool {
        v.iter().any(P::test)
    }
}
impl<T: PartialOrd> Predicate<[T]> for Sorted {
    fn test(v: &[T]) -> bool {
        v.windows(2).all(|w| w[0] <= w[1])
    }
}
#[cfg(feature = "alloc")]
impl<T, P: Predicate<T>> Predicate<alloc::vec::Vec<T>> for Each<P> {
    fn test(v: &alloc::vec::Vec<T>) -> bool {
        v.iter().all(P::test)
    }
}
#[cfg(feature = "alloc")]
impl<T, P: Predicate<T>> Predicate<alloc::vec::Vec<T>> for Exists<P> {
    fn test(v: &alloc::vec::Vec<T>) -> bool {
        v.iter().any(P::test)
    }
}
#[cfg(feature = "alloc")]
impl<T: PartialOrd> Predicate<alloc::vec::Vec<T>> for Sorted {
    fn test(v: &alloc::vec::Vec<T>) -> bool {
        <Self as Predicate<[T]>>::test(v)
    }
}
macro_rules! integers { ($($t:ty),*) => { $(
    impl Predicate<$t> for Positive { fn test(v: &$t) -> bool { *v > 0 } }
    impl Predicate<$t> for NonZero { fn test(v: &$t) -> bool { *v != 0 } }
    impl Predicate<$t> for Even { fn test(v: &$t) -> bool { *v % 2 == 0 } }
    impl Predicate<$t> for Odd { fn test(v: &$t) -> bool { *v % 2 != 0 } }
    impl<const L:u128,const H:u128> Predicate<$t> for InRange<L,H> {
        fn test(v: &$t) -> bool { u128::try_from(*v).is_ok_and(|n| L <= n && n <= H) }
    }
)* }; }
integers!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
macro_rules! floats { ($($t:ty),*) => { $(
    impl Predicate<$t> for Positive { fn test(v: &$t) -> bool { *v > 0.0 } }
    impl Predicate<$t> for NonZero { fn test(v: &$t) -> bool { !v.is_nan() && *v != 0.0 } }
    impl Predicate<$t> for Finite { fn test(v: &$t) -> bool { v.is_finite() } }
)* }; }
floats!(f32, f64);
