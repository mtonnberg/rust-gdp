use rust_gdp::{with_name,predicates::*}; fn main() { let r=with_name("ok",|n| { let p=n.check::<NonEmpty>().unwrap(); n.refine(p) }); assert_eq!(r.into_inner(),"ok"); }
