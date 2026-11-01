use rust_gdp::{name,predicates::Positive}; fn main() { name!(a=1); name!(b=0); b.refine(a.check::<rust_gdp::Positive<'_>>().unwrap()); loop {} }
