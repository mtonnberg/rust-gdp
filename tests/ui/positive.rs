#![forbid(unsafe_code)]
use rust_gdp::{with_name,predicates::*}; fn main() { let r=with_name("ok",|n| { let p=n.check::<rust_gdp::NonEmpty<'_>>().unwrap(); n.refine(p) }); assert_eq!(r.into_inner(),"ok"); }
fn lexical() { rust_gdp::name!(v=2); let _=v.check::<rust_gdp::Positive<'_>>(); }
