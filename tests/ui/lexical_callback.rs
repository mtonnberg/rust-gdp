use rust_gdp::{name,with_name,predicates::Positive};
fn main() { name!(a=1); let p=a.check::<Positive>().unwrap(); with_name(0,|b| { b.refine(p); }); }
