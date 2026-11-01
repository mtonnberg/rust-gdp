use rust_gdp::{name,with_name,predicates::Positive};
fn main() { with_name(1, |a| { let p=a.check::<Positive>().unwrap(); name!(b=0); b.refine(p); }); }
