use rust_gdp::{name,predicates::NonEmpty}; fn main() { name!(a="ok"); name!(b=""); let p=a.check::<rust_gdp::NonEmpty<'_>>().unwrap(); b.refine(p); }
