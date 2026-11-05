use rust_gdp::{with_name, Refined, predicates::*}; fn main() { with_name(2, |a| { let p = a.check::<rust_gdp::Positive<'_>>().unwrap(); let _: Refined<_,Odd> = a.refine(p); }); }
