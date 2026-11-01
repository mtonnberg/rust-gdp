use rust_gdp::{Proof,Holds,predicates::Positive}; fn shorten<'a:'b,'b>(p:Proof<Holds<'a,Positive>>)->Proof<Holds<'b,Positive>> { p } fn main() {}
