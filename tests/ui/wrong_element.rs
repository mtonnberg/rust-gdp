use rust_gdp::{Refined,CheckedRef,predicates::*};
fn need(_:CheckedRef<'_,i32,Odd>) {}
fn main() { let xs=[2]; let all=Refined::<_,Each<Positive>>::new(&xs[..]).unwrap(); need(all.element(0).unwrap()); }
