use rust_gdp::{name,predicates::NonEmpty}; fn main() { name!(a="ok"); name!(b=""); let p=a.check::<NonEmpty>().unwrap(); b.refine(p); }
