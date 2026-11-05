use rust_gdp::{name, And, Ascii, Named, NonEmpty, Proof};

fn greet<'n>(name: &Named<'n, &str>, _proof: Proof<And<NonEmpty<'n>, Ascii<'n>>>) {
    println!("Hello, {}!", name.get());
}

fn main() {
    name!(name = "Ada");
    let nonempty = name.check::<NonEmpty<'_>>().unwrap();
    let ascii = name.check::<Ascii<'_>>().unwrap();
    greet(&name, nonempty.and(ascii));
    // Or validate the conjunction directly:
    let both = name.check::<And<NonEmpty<'_>, Ascii<'_>>>().unwrap();
    greet(&name, both);
    let validated = name.refine(both);
    assert_eq!(validated.into_inner(), "Ada");
}
