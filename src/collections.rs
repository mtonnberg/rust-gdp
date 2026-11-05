//! Transport universally quantified element facts without rechecking elements.
use crate::{predicates::Each, CheckedRef, Refined};
use core::marker::PhantomData;

impl<T, P> Refined<&[T], Each<P>> {
    /// Borrow checked elements. Empty input yields no evidence for an element.
    pub fn iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = CheckedRef<'_, T, P>> + ExactSizeIterator {
        self.value.iter().map(|value| CheckedRef {
            value,
            predicate: PhantomData,
        })
    }
    /// Look up one element, preserving its checked property on success.
    pub fn element(&self, index: usize) -> Option<CheckedRef<'_, T, P>> {
        self.value.get(index).map(|value| CheckedRef {
            value,
            predicate: PhantomData,
        })
    }
}

#[cfg(feature = "alloc")]
impl<T, P> Refined<alloc::vec::Vec<T>, Each<P>> {
    /// Borrow checked elements without repeating the element predicate.
    pub fn iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = CheckedRef<'_, T, P>> + ExactSizeIterator {
        self.value.iter().map(|value| CheckedRef {
            value,
            predicate: PhantomData,
        })
    }
    /// Look up an element with its existing evidence.
    pub fn element(&self, index: usize) -> Option<CheckedRef<'_, T, P>> {
        self.value.get(index).map(|value| CheckedRef {
            value,
            predicate: PhantomData,
        })
    }
    /// Move out individually refined elements, without checking again.
    pub fn into_elements(
        self,
    ) -> impl DoubleEndedIterator<Item = Refined<T, P>> + ExactSizeIterator {
        self.value.into_iter().map(|value| Refined {
            value,
            predicate: PhantomData,
        })
    }
    /// Map checked elements to checked results, preserving all output evidence.
    /// Only the callback performs validation; there is no second pass over outputs.
    pub fn map_checked<U, Q>(
        self,
        mut f: impl FnMut(Refined<T, P>) -> Refined<U, Q>,
    ) -> Refined<alloc::vec::Vec<U>, Each<Q>> {
        let value = self.into_elements().map(|v| f(v).into_inner()).collect();
        Refined {
            value,
            predicate: PhantomData,
        }
    }
}

/// Validate each element once, retaining only those that satisfy `P`.
/// This allocates a result vector using the usual iterator collection behavior.
#[cfg(feature = "alloc")]
pub fn filter_checked<T, P: crate::Predicate<T>>(
    values: impl IntoIterator<Item = T>,
) -> Refined<alloc::vec::Vec<T>, Each<P>> {
    let value = values.into_iter().filter(P::test).collect();
    Refined {
        value,
        predicate: PhantomData,
    }
}
