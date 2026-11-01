use rust_gdp::{with_name,predicates::Positive}; fn main() { with_name(1u32, |n| { let p=n.check::<Positive>().unwrap(); let _ = n.into_inner(); n.refine(p); }); }
