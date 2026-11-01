use rust_gdp::{Refined,predicates::Positive}; fn main() { let p=Refined::<_,Positive>::new(1).unwrap(); let _ = p.clone(); }
