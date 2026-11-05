use rust_gdp::with_name; fn main() { with_name(String::from("x"), |n| { n.get().clear(); }); }
