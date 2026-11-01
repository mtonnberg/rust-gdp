use rust_gdp::{name,Named}; fn main() { name!(a=1); let _:Named<'static,_>=a; }
