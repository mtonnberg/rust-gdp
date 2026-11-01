//! Binary checks on two independently named immutable values.
use crate::{Brand, Named, Proof};
use core::marker::PhantomData;

/// A decidable binary specification. Custom implementations are trusted specifications.
pub trait Relation<A: ?Sized, B: ?Sized> {
    /// Decide the relation for these values.
    fn test(left: &A, right: &B) -> bool;
}
/// Equality according to `PartialEq`.
pub enum Equal {}
/// Strict ordering according to `PartialOrd`; incomparable values reject the check.
pub enum Less {}
/// Nonstrict ordering according to `PartialOrd`.
pub enum LessEqual {}
/// Evidence that relation `R` held for two names, in this order.
pub struct Related<'a, 'b, R>(Brand<'a>, Brand<'b>, PhantomData<fn() -> R>);
impl<A: ?Sized + PartialEq<B>, B: ?Sized> Relation<A, B> for Equal {
    fn test(a: &A, b: &B) -> bool {
        a == b
    }
}
impl<A: ?Sized + PartialOrd<B>, B: ?Sized> Relation<A, B> for Less {
    fn test(a: &A, b: &B) -> bool {
        a < b
    }
}
impl<A: ?Sized + PartialOrd<B>, B: ?Sized> Relation<A, B> for LessEqual {
    fn test(a: &A, b: &B) -> bool {
        a <= b
    }
}
impl<'a, A> Named<'a, A> {
    /// Check a binary relation once, tying the result to both names.
    #[inline]
    pub fn relate<'b, B, R: Relation<A, B>>(
        &self,
        right: &Named<'b, B>,
    ) -> Option<Proof<Related<'a, 'b, R>>> {
        R::test(self.get(), right.get()).then(Proof::new)
    }
}

/// An existential package of two values and their established relation.
/// The proof marker occupies no storage. Unpacking introduces fresh names and
/// restores the relation without repeating its check.
pub struct RelatedPair<A, B, R> {
    left: A,
    right: B,
    relation: PhantomData<fn() -> R>,
}
impl<'a, A> Named<'a, A> {
    /// Package both values with existing evidence about exactly these names.
    #[inline(always)]
    pub fn pack_with<'b, B, R>(
        self,
        right: Named<'b, B>,
        _: Proof<Related<'a, 'b, R>>,
    ) -> RelatedPair<A, B, R> {
        RelatedPair {
            left: self.into_inner(),
            right: right.into_inner(),
            relation: PhantomData,
        }
    }
}
impl<A, B, R> RelatedPair<A, B, R> {
    /// Unpack with fresh names and the stored relation, without rechecking.
    #[inline(always)]
    pub fn with_names<O>(
        self,
        f: impl for<'a, 'b> FnOnce(Named<'a, A>, Named<'b, B>, Proof<Related<'a, 'b, R>>) -> O,
    ) -> O {
        crate::with_name(self.left, |a| {
            crate::with_name(self.right, |b| f(a, b, Proof::new()))
        })
    }
    /// Erase the relation and recover both values.
    #[inline(always)]
    pub fn into_inner(self) -> (A, B) {
        (self.left, self.right)
    }
}
