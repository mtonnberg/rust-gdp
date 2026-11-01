use rust_gdp::{name,with_name,predicates::Positive};
fn main() { name!(a=1); let p=a.check::<rust_gdp::Positive<'_>>().unwrap(); with_name(0,|b| { b.refine(p); }); }
