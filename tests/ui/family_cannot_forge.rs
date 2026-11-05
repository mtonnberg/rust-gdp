use rust_gdp::{Predicate,PredicateFamily,NonEmpty};
enum Always {}
impl Predicate<&str> for Always { fn test(_:&&str)->bool { true } }
impl PredicateFamily for Always { type Fact<'n> = NonEmpty<'n>; }
fn main() {}
