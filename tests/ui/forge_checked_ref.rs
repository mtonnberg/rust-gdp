use rust_gdp::{CheckedRef,predicates::Positive};
fn main() { let _ = CheckedRef::<_,Positive> { value: &0, predicate: core::marker::PhantomData }; }
