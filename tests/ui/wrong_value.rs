use rust_gdp::{with_name, predicates::NonEmpty}; fn main() { with_name("yes", |a| with_name("", |b| { let p = a.check::<rust_gdp::NonEmpty<'_>>().unwrap(); b.refine(p); })); }
