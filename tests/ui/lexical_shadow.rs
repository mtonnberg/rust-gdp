use rust_gdp::{name,predicates::Positive}; fn main() { name!(a=1); let p=a.check::<Positive>().unwrap(); name!(a=0); a.refine(p); }
