use rust_gdp::{with_name,predicates::NonEmpty};
fn main() { with_name("yes", |x| { let p=x.check::<rust_gdp::NonEmpty<'_>>().unwrap(); with_name("", |x| { x.refine(p); }); }); }
