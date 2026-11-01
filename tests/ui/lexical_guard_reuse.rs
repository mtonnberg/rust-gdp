use rust_gdp::{__generativity,Named}; fn main() { __generativity::make_guard!(g); let _=Named::from_guard(1,g); let _=Named::from_guard(2,g); }
