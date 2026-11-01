use rust_gdp::{name,predicates::Positive}; fn main() { let mut saved=None; for i in 0..2 { name!(a=i); if let Some(p)=saved { a.refine(p); } else { saved=a.check::<rust_gdp::Positive<'_>>(); } } }
