//! Ghosts of Departed Proofs for Rust: check once, carry evidence in types.
//!
//! ```
//! use rust_gdp::{with_name, Proof, Named};
//! use rust_gdp::NonEmpty;
//! fn accept<'n>(s: &Named<'n, &str>, _: Proof<NonEmpty<'n>>) -> usize {
//!     s.get().len()
//! }
//! with_name("hello", |s| {
//!     let proof = s.check::<NonEmpty<'_>>().unwrap();
//!     assert_eq!(accept(&s, proof), 5);
//! });
//! ```
//! Proof constructors are private; each invocation introduces a fresh invariant name.
//!
//! ```compile_fail
//! use rust_gdp::{with_name, Proof};
//! use rust_gdp::NonEmpty;
//! with_name("yes", |a| with_name("", |b| {
//!     let p = a.check::<NonEmpty<'_>>().unwrap();
//!     b.refine(p); // evidence belongs to a, not b
//! }));
//! ```
//! ```compile_fail
//! use rust_gdp::with_name;
//! use rust_gdp::NonEmpty;
//! let escaped = with_name("yes", |x| x.check::<NonEmpty<'_>>().unwrap());
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
pub mod facts;
pub use facts::*;
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

/// A proposition about exactly the value branded by `'n`.
///
/// The associated specification decides this fact. Custom implementations define
/// trusted domain facts; implementations for built-in fact types are owned by
/// this crate, preventing downstream code from retargeting their brands.
pub trait Fact<'n> {
    /// The unbranded specification that validates this proposition.
    type Predicate;
}

/// Connect a persistent specification to its branded proposition family.
/// The equality bound prevents a custom family from manufacturing built-in facts.
pub trait PredicateFamily {
    /// This specification instantiated at the supplied fresh brand.
    type Fact<'n>: Fact<'n, Predicate = Self>;
}
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
/// Name a value in the current lexical scope with a fresh hidden brand.
///
/// ```
/// use rust_gdp::{name, NonEmpty};
/// name!(message = "hello");
/// let proof = message.check::<NonEmpty<'_>>().unwrap();
/// assert_eq!(message.refine(proof).into_inner(), "hello");
/// ```
///
/// The hidden generic parameter is an invariant lifetime, generated once per
/// evaluation (including loop iterations). It cannot be chosen or reproduced in
/// safe code. Names and proofs are bounded by the enclosing lexical scope.
///
/// ```compile_fail
/// use rust_gdp::{name, NonEmpty};
/// name!(a = "hello");
/// name!(b = "");
/// b.refine(a.check::<NonEmpty<'_>>().unwrap());
/// ```
///
/// ```compile_fail
/// use rust_gdp::name;
/// let escaped = { name!(value = 1); value };
/// ```
#[macro_export]
macro_rules! name {
    ($name:ident = $value:expr $(;)?) => {
        let __rust_gdp_value = $value;
        $crate::__generativity::make_guard!(__rust_gdp_guard);
        let $name = $crate::Named::from_guard(__rust_gdp_value, __rust_gdp_guard);
    };
    (let $name:ident = $value:expr $(;)?) => {
        $crate::name!($name = $value);
    };
}

// Re-export for hygienic expansion, including when the dependency is renamed.
#[doc(hidden)]
pub use generativity as __generativity;

impl<'n, T> Named<'n, T> {
    /// Bind one value to a unique generativity guard, consuming the guard.
    ///
    /// Prefer [`name!`] unless interoperating with a caller's `generativity`
    /// guard. The guard is linear: an `Id` (which is copyable) is insufficient.
    #[inline(always)]
    pub fn from_guard(value: T, _guard: generativity::Guard<'n>) -> Self {
        Self {
            value,
            brand: PhantomData,
        }
    }

    /// Read the value without changing its identity.
    #[inline(always)]
    pub fn get(&self) -> &T {
        &self.value
    }
    /// Run a predicate once and obtain evidence on success.
    #[inline]
    pub fn check<F: Fact<'n>>(&self) -> Option<Proof<F>>
    where
        F::Predicate: Predicate<T>,
    {
        F::Predicate::test(&self.value).then(Proof::new)
    }
    /// Attach established evidence to the owned value without rechecking.
    #[inline(always)]
    pub fn refine<F: Fact<'n>>(self, _: Proof<F>) -> Refined<T, F::Predicate> {
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
    pub fn with_name<R>(self, f: impl for<'n> FnOnce(Named<'n, T>, Proof<P::Fact<'n>>) -> R) -> R
    where
        P: PredicateFamily,
    {
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
impl<'n, T> Named<'n, T> {
    /// Decide a predicate, retaining positive or negative evidence in the branch.
    #[inline]
    pub fn decide<F: Fact<'n>>(&self) -> Result<Proof<F>, Proof<facts::Not<F>>>
    where
        F::Predicate: Predicate<T>,
    {
        if F::Predicate::test(&self.value) {
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
