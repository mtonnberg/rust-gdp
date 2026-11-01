//! Trusted standard transformations that establish facts by construction.
use crate::{predicates::Trimmed, Refined};
use core::marker::PhantomData;

/// Trim Unicode whitespace, establishing `Trimmed` without a second scan to check it.
pub fn trim(value: &str) -> Refined<&str, Trimmed> {
    Refined {
        value: value.trim(),
        predicate: PhantomData,
    }
}

/// Sort values, establishing `Sorted` without a validation pass.
/// As with Rust's sorting APIs, `Ord` implementations must obey their documented laws.
#[cfg(feature = "alloc")]
pub fn sort<T: Ord>(
    mut values: alloc::vec::Vec<T>,
) -> Refined<alloc::vec::Vec<T>, crate::predicates::Sorted> {
    values.sort();
    Refined {
        value: values,
        predicate: PhantomData,
    }
}
