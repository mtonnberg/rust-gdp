use rust_gdp::{Refined,predicates::Positive}; fn main() { let mut r=Refined::<_,Positive>::new(1u32).unwrap(); let _ = r.get_mut(); }
