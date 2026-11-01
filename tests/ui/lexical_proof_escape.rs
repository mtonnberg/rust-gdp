use rust_gdp::{name,predicates::Positive}; fn main() { let _ = { name!(a=1); a.check::<rust_gdp::Positive<'_>>().unwrap() }; }
