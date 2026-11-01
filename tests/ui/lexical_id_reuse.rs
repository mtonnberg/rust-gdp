use rust_gdp::{__generativity,Named}; fn main() { __generativity::make_guard!(g); let id:__generativity::Id= g.into(); let _=Named::from_guard(1,id); }
