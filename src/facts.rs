//! Value-indexed propositions. Predicate implementations live in [`crate::predicates`].
use crate::{predicates, Brand, Fact, PredicateFamily};
use core::marker::PhantomData;

/// The named value satisfies [`predicates::NonEmpty`].
pub struct NonEmpty<'n>(Brand<'n>);
impl<'n> Fact<'n> for NonEmpty<'n> {
    type Predicate = predicates::NonEmpty;
}
impl PredicateFamily for predicates::NonEmpty {
    type Fact<'n> = NonEmpty<'n>;
}
/// The named value satisfies [`predicates::Ascii`].
pub struct Ascii<'n>(Brand<'n>);
impl<'n> Fact<'n> for Ascii<'n> {
    type Predicate = predicates::Ascii;
}
impl PredicateFamily for predicates::Ascii {
    type Fact<'n> = Ascii<'n>;
}
/// The named value satisfies [`predicates::Utf8`].
pub struct Utf8<'n>(Brand<'n>);
impl<'n> Fact<'n> for Utf8<'n> {
    type Predicate = predicates::Utf8;
}
impl PredicateFamily for predicates::Utf8 {
    type Fact<'n> = Utf8<'n>;
}
/// The named value satisfies [`predicates::Trimmed`].
pub struct Trimmed<'n>(Brand<'n>);
impl<'n> Fact<'n> for Trimmed<'n> {
    type Predicate = predicates::Trimmed;
}
impl PredicateFamily for predicates::Trimmed {
    type Fact<'n> = Trimmed<'n>;
}
/// The named value satisfies [`predicates::Positive`].
pub struct Positive<'n>(Brand<'n>);
impl<'n> Fact<'n> for Positive<'n> {
    type Predicate = predicates::Positive;
}
impl PredicateFamily for predicates::Positive {
    type Fact<'n> = Positive<'n>;
}
/// The named value satisfies [`predicates::NonZero`].
pub struct NonZero<'n>(Brand<'n>);
impl<'n> Fact<'n> for NonZero<'n> {
    type Predicate = predicates::NonZero;
}
impl PredicateFamily for predicates::NonZero {
    type Fact<'n> = NonZero<'n>;
}
/// The named value satisfies [`predicates::Finite`].
pub struct Finite<'n>(Brand<'n>);
impl<'n> Fact<'n> for Finite<'n> {
    type Predicate = predicates::Finite;
}
impl PredicateFamily for predicates::Finite {
    type Fact<'n> = Finite<'n>;
}
/// The named value satisfies [`predicates::Even`].
pub struct Even<'n>(Brand<'n>);
impl<'n> Fact<'n> for Even<'n> {
    type Predicate = predicates::Even;
}
impl PredicateFamily for predicates::Even {
    type Fact<'n> = Even<'n>;
}
/// The named value satisfies [`predicates::Odd`].
pub struct Odd<'n>(Brand<'n>);
impl<'n> Fact<'n> for Odd<'n> {
    type Predicate = predicates::Odd;
}
impl PredicateFamily for predicates::Odd {
    type Fact<'n> = Odd<'n>;
}
/// The named value satisfies [`predicates::IsSome`].
pub struct IsSome<'n>(Brand<'n>);
impl<'n> Fact<'n> for IsSome<'n> {
    type Predicate = predicates::IsSome;
}
impl PredicateFamily for predicates::IsSome {
    type Fact<'n> = IsSome<'n>;
}
/// The named value satisfies [`predicates::IsNone`].
pub struct IsNone<'n>(Brand<'n>);
impl<'n> Fact<'n> for IsNone<'n> {
    type Predicate = predicates::IsNone;
}
impl PredicateFamily for predicates::IsNone {
    type Fact<'n> = IsNone<'n>;
}
/// The named value satisfies [`predicates::IsOk`].
pub struct IsOk<'n>(Brand<'n>);
impl<'n> Fact<'n> for IsOk<'n> {
    type Predicate = predicates::IsOk;
}
impl PredicateFamily for predicates::IsOk {
    type Fact<'n> = IsOk<'n>;
}
/// The named value satisfies [`predicates::IsErr`].
pub struct IsErr<'n>(Brand<'n>);
impl<'n> Fact<'n> for IsErr<'n> {
    type Predicate = predicates::IsErr;
}
impl PredicateFamily for predicates::IsErr {
    type Fact<'n> = IsErr<'n>;
}
/// The named value satisfies [`predicates::Sorted`].
pub struct Sorted<'n>(Brand<'n>);
impl<'n> Fact<'n> for Sorted<'n> {
    type Predicate = predicates::Sorted;
}
impl PredicateFamily for predicates::Sorted {
    type Fact<'n> = Sorted<'n>;
}
/// The named value satisfies [`predicates::Length`].
pub struct Length<'n, const N: usize>(Brand<'n>);
impl<'n, const N: usize> Fact<'n> for Length<'n, N> {
    type Predicate = predicates::Length<N>;
}
impl<const N: usize> PredicateFamily for predicates::Length<N> {
    type Fact<'n> = Length<'n, N>;
}
/// The named value satisfies [`predicates::MinLength`].
pub struct MinLength<'n, const N: usize>(Brand<'n>);
impl<'n, const N: usize> Fact<'n> for MinLength<'n, N> {
    type Predicate = predicates::MinLength<N>;
}
impl<const N: usize> PredicateFamily for predicates::MinLength<N> {
    type Fact<'n> = MinLength<'n, N>;
}
/// The named value satisfies [`predicates::MaxLength`].
pub struct MaxLength<'n, const N: usize>(Brand<'n>);
impl<'n, const N: usize> Fact<'n> for MaxLength<'n, N> {
    type Predicate = predicates::MaxLength<N>;
}
impl<const N: usize> PredicateFamily for predicates::MaxLength<N> {
    type Fact<'n> = MaxLength<'n, N>;
}
/// The named integer is inside an inclusive nonnegative interval.
pub struct InRange<'n, const L: u128, const H: u128>(Brand<'n>);
impl<'n, const L: u128, const H: u128> Fact<'n> for InRange<'n, L, H> {
    type Predicate = predicates::InRange<L, H>;
}
impl<const L: u128, const H: u128> PredicateFamily for predicates::InRange<L, H> {
    type Fact<'n> = InRange<'n, L, H>;
}
/// The named collection satisfies [`predicates::Each<P>`].
pub struct Each<'n, P>(Brand<'n>, PhantomData<fn() -> P>);
impl<'n, P> Fact<'n> for Each<'n, P> {
    type Predicate = predicates::Each<P>;
}
impl<P> PredicateFamily for predicates::Each<P> {
    type Fact<'n> = Each<'n, P>;
}
/// The named collection satisfies [`predicates::Exists<P>`].
pub struct Exists<'n, P>(Brand<'n>, PhantomData<fn() -> P>);
impl<'n, P> Fact<'n> for Exists<'n, P> {
    type Predicate = predicates::Exists<P>;
}
impl<P> PredicateFamily for predicates::Exists<P> {
    type Fact<'n> = Exists<'n, P>;
}
/// Negation of a decidable branded fact.
pub struct Not<F>(PhantomData<fn() -> F>);
impl<'n, F: Fact<'n>> Fact<'n> for Not<F> {
    type Predicate = predicates::Not<F::Predicate>;
}
impl<P: PredicateFamily> PredicateFamily for predicates::Not<P> {
    type Fact<'n> = Not<P::Fact<'n>>;
}
impl<'n, A: Fact<'n>, B: Fact<'n>> Fact<'n> for crate::And<A, B> {
    type Predicate = predicates::All<A::Predicate, B::Predicate>;
}
impl<P: PredicateFamily, Q: PredicateFamily> PredicateFamily for predicates::All<P, Q> {
    type Fact<'n> = crate::And<P::Fact<'n>, Q::Fact<'n>>;
}
impl<'n, A: Fact<'n>, B: Fact<'n>> Fact<'n> for crate::Or<A, B> {
    type Predicate = predicates::Any<A::Predicate, B::Predicate>;
}
impl<P: PredicateFamily, Q: PredicateFamily> PredicateFamily for predicates::Any<P, Q> {
    type Fact<'n> = crate::Or<P::Fact<'n>, Q::Fact<'n>>;
}
