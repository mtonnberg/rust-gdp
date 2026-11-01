//! Ghosts of Departed Proofs for Rust: check once, carry evidence in types.
//!
//! ```
//! use rust_gdp::{with_name, Proof, Holds, Named};
//! use rust_gdp::predicates::NonEmpty;
//! fn accept<'n>(s: &Named<'n, &str>, _: Proof<Holds<'n, NonEmpty>>) -> usize {
//!     s.get().len()
//! }
//! with_name("hello", |s| {
//!     let proof = s.check::<NonEmpty>().unwrap();
//!     assert_eq!(accept(&s, proof), 5);
//! });
//! ```
//! Proof constructors are private; each invocation introduces a fresh invariant name.
//!
//! ```compile_fail
//! use rust_gdp::{with_name, Proof, Holds};
//! use rust_gdp::predicates::NonEmpty;
//! with_name("yes", |a| with_name("", |b| {
//!     let p = a.check::<NonEmpty>().unwrap();
//!     b.refine(p); // evidence belongs to a, not b
//! }));
//! ```
//! ```compile_fail
//! use rust_gdp::with_name;
//! use rust_gdp::predicates::NonEmpty;
//! let escaped = with_name("yes", |x| x.check::<NonEmpty>().unwrap());
//! ```
//! ```compile_fail
//! use rust_gdp::Proof;
//! let forged: Proof<()> = Default::default();
//! ```
//! This is a library of checked evidence and constructive combinators, not an SMT
//! solver or a verifier for arbitrary Rust. Custom predicates are specifications:
//! their meaning and correctness remain the implementor's responsibility. No proof
//! authorizes unsafe memory access. See the README for the trust model and costs.
#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

use core::marker::PhantomData;
pub mod collections;
pub mod predicates;
pub mod relations;
pub mod transform;
pub mod uint;

/// Invariant, generative brand. Function variance prevents shortening a name.
type Brand<'n> = PhantomData<fn(&'n ()) -> &'n ()>;

/// Zero-sized evidence of proposition `P`. Cannot be constructed by callers.
#[repr(transparent)]
pub struct Proof<P>(PhantomData<fn() -> P>);
impl<P> Copy for Proof<P> {}
impl<P> Clone for Proof<P> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<P> core::fmt::Debug for Proof<P> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Proof")
    }
}
impl<P> Proof<P> {
    const fn new() -> Self {
        Self(PhantomData)
    }
}

/// Proposition: predicate `P` held when checked for the immutable value named `'n`.
pub struct Holds<'n, P>(Brand<'n>, PhantomData<fn() -> P>);
/// Constructive conjunction.
pub struct And<A, B>(PhantomData<fn() -> (A, B)>);
/// Constructive disjunction. Its erased evidence does not retain which branch held.
pub struct Or<A, B>(PhantomData<fn() -> (A, B)>);
/// Truth, always inhabited.
pub enum True {}
/// Falsehood, with no public evidence constructor.
pub enum False {}

/// Evidence of truth.
#[inline(always)]
pub const fn truth() -> Proof<True> {
    Proof::new()
}
impl<A> Proof<A> {
    /// Introduce conjunction. No checks, allocations, or stored data.
    #[inline(always)]
    pub const fn and<B>(self, _: Proof<B>) -> Proof<And<A, B>> {
        Proof::new()
    }
    /// Introduce disjunction on the left; branch information is erased.
    #[inline(always)]
    pub const fn or_left<B>(self) -> Proof<Or<A, B>> {
        Proof::new()
    }
    /// Introduce disjunction on the right; branch information is erased.
    #[inline(always)]
    pub const fn or_right<B>(self) -> Proof<Or<B, A>> {
        Proof::new()
    }
}
impl<A, B> Proof<And<A, B>> {
    /// Eliminate conjunction.
    #[inline(always)]
    pub const fn split(self) -> (Proof<A>, Proof<B>) {
        (Proof::new(), Proof::new())
    }
}
impl Proof<False> {
    /// Eliminate falsehood. Unreachable using the public API.
    pub fn absurd<P>(self) -> Proof<P> {
        Proof::new()
    }
}

/// A boolean specification. Implementations should be deterministic and pure.
///
/// Custom implementations are a logical trust boundary, not a memory-safety
/// boundary. Do not use changing global state or interior mutation to define a
/// persistent property. Built-in predicates only observe immutable structure.
pub trait Predicate<T: ?Sized> {
    /// Decide whether the property holds.
    fn test(value: &T) -> bool;
}

/// Immutable owned value associated with a fresh, scope-local name.
///
/// There is deliberately no mutable accessor or `Clone` implementation.
#[repr(transparent)]
pub struct Named<'n, T> {
    value: T,
    brand: Brand<'n>,
}

/// Give a value a fresh type-level name, scoped to the callback.
///
/// Names and proofs cannot escape; ordinary results and refined values can.
#[inline(always)]
pub fn with_name<T, R>(value: T, f: impl for<'n> FnOnce(Named<'n, T>) -> R) -> R {
    f(Named {
        value,
        brand: PhantomData,
    })
}
impl<'n, T> Named<'n, T> {
    /// Read the value without changing its identity.
    #[inline(always)]
    pub fn get(&self) -> &T {
        &self.value
    }
    /// Run a predicate once and obtain evidence on success.
    #[inline]
    pub fn check<P: Predicate<T>>(&self) -> Option<Proof<Holds<'n, P>>> {
        P::test(&self.value).then(Proof::new)
    }
    /// Attach established evidence to the owned value without rechecking.
    #[inline(always)]
    pub fn refine<P>(self, _: Proof<Holds<'n, P>>) -> Refined<T, P> {
        Refined {
            value: self.value,
            predicate: PhantomData,
        }
    }
    /// Recover ownership, consuming the name. Existing proofs cannot name a new value.
    #[inline(always)]
    pub fn into_inner(self) -> T {
        self.value
    }
}

/// A checked refinement with exactly the layout of `T`.
///
/// No mutable access, unchecked constructors, or `DerefMut` are provided. The
/// predicate is checked at construction, never implicitly during reads or moves.
/// For custom predicates, avoid observable interior mutation (see [`Predicate`]).
///
/// ```compile_fail
/// use rust_gdp::{Refined, predicates::NonEmpty};
/// let mut s = Refined::<_, NonEmpty>::new(String::from("x")).unwrap();
/// s.get_mut().clear();
/// ```
#[repr(transparent)]
pub struct Refined<T, P> {
    value: T,
    predicate: PhantomData<fn() -> P>,
}
impl<T, P: Predicate<T>> Refined<T, P> {
    /// Check an untrusted value, returning it unchanged on failure.
    #[inline]
    pub fn new(value: T) -> Result<Self, T> {
        if P::test(&value) {
            Ok(Self {
                value,
                predicate: PhantomData,
            })
        } else {
            Err(value)
        }
    }
    /// Transform a value, discarding old evidence and checking the new value.
    #[inline]
    pub fn try_map<U, Q: Predicate<U>>(self, f: impl FnOnce(T) -> U) -> Result<Refined<U, Q>, U> {
        Refined::new(f(self.value))
    }
}
impl<T, P> Refined<T, P> {
    /// Borrow without rechecking.
    #[inline(always)]
    pub fn get(&self) -> &T {
        &self.value
    }
    /// Erase the refinement, recovering the value without checks.
    #[inline(always)]
    pub fn into_inner(self) -> T {
        self.value
    }
    /// Name this value and recover scoped evidence, without rechecking.
    #[inline(always)]
    pub fn with_name<R>(self, f: impl for<'n> FnOnce(Named<'n, T>, Proof<Holds<'n, P>>) -> R) -> R {
        with_name(self.value, |n| f(n, Proof::new()))
    }
}
impl<T: core::fmt::Debug, P> core::fmt::Debug for Refined<T, P> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Refined").field(&self.value).finish()
    }
}
// Clone is intentionally absent: arbitrary Clone implementations need not preserve predicates.

/// A constructive implication, built using the structural rules below.
pub struct Implies<A, B>(PhantomData<fn() -> (A, B)>);
/// The identity theorem `A -> A`.
#[inline(always)]
pub const fn identity<A>() -> Proof<Implies<A, A>> {
    Proof::new()
}
/// The projection theorem `(A and B) -> A`.
#[inline(always)]
pub const fn and_left<A, B>() -> Proof<Implies<And<A, B>, A>> {
    Proof::new()
}
/// The projection theorem `(A and B) -> B`.
#[inline(always)]
pub const fn and_right<A, B>() -> Proof<Implies<And<A, B>, B>> {
    Proof::new()
}
impl<A, B> Proof<Implies<A, B>> {
    /// Modus ponens, with no runtime function to call.
    #[inline(always)]
    pub const fn apply(self, _: Proof<A>) -> Proof<B> {
        Proof::new()
    }
    /// Transitivity of implication.
    #[inline(always)]
    pub const fn then<C>(self, _: Proof<Implies<B, C>>) -> Proof<Implies<A, C>> {
        Proof::new()
    }
}
impl<A, B> Proof<Or<A, B>> {
    /// Eliminate an erased disjunction when both branches imply the same result.
    #[inline(always)]
    pub const fn cases<C>(self, _: Proof<Implies<A, C>>, _: Proof<Implies<B, C>>) -> Proof<C> {
        Proof::new()
    }
}
impl<'n, P> Proof<Holds<'n, P>> {
    /// Combine two checks on the same named value into a predicate conjunction.
    #[inline(always)]
    pub const fn both<Q>(self, _: Proof<Holds<'n, Q>>) -> Proof<Holds<'n, predicates::All<P, Q>>> {
        Proof::new()
    }
}
impl<'n, P, Q> Proof<Holds<'n, predicates::All<P, Q>>> {
    /// Recover both constituent predicate proofs.
    #[inline(always)]
    pub const fn components(self) -> (Proof<Holds<'n, P>>, Proof<Holds<'n, Q>>) {
        (Proof::new(), Proof::new())
    }
}
impl<'n, T> Named<'n, T> {
    /// Decide a predicate, retaining positive or negative evidence in the branch.
    #[inline]
    pub fn decide<P: Predicate<T>>(
        &self,
    ) -> Result<Proof<Holds<'n, P>>, Proof<Holds<'n, predicates::Not<P>>>> {
        if P::test(&self.value) {
            Ok(Proof::new())
        } else {
            Err(Proof::new())
        }
    }
}

/// A borrowed checked value. Same layout as `&T`; reads do not repeat checks.
/// Like other evidence, custom predicates must not depend on changing aliased state.
#[repr(transparent)]
pub struct CheckedRef<'a, T: ?Sized, P> {
    value: &'a T,
    predicate: PhantomData<fn() -> P>,
}
impl<T: ?Sized, P> Copy for CheckedRef<'_, T, P> {}
impl<T: ?Sized, P> Clone for CheckedRef<'_, T, P> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, T: ?Sized, P> CheckedRef<'a, T, P> {
    /// Borrow the checked value without rechecking.
    #[inline(always)]
    pub fn get(self) -> &'a T {
        self.value
    }
}
impl<T, P> Refined<T, P> {
    /// Borrow the value together with its established predicate.
    #[inline(always)]
    pub fn as_checked(&self) -> CheckedRef<'_, T, P> {
        CheckedRef {
            value: &self.value,
            predicate: PhantomData,
        }
    }
}
