use rust_gdp::{Fact,NonEmpty,predicates};
impl<'a,'b> Fact<'a> for NonEmpty<'b> { type Predicate=predicates::NonEmpty; }
fn main() {}
