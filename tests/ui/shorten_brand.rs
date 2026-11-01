use rust_gdp::{Proof,predicates::Positive}; fn shorten<'a:'b,'b>(p:Proof<rust_gdp::Positive<'a>>)->Proof<rust_gdp::Positive<'b>> { p } fn main() {}
