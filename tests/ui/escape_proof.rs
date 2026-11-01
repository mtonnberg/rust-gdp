use rust_gdp::{with_name, predicates::Positive}; fn main() { let _ = with_name(1, |n| n.check::<Positive>().unwrap()); }
