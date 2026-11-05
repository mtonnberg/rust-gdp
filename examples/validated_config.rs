use rust_gdp::{predicates::*, uint::U13, Refined};

type ServiceName = Refined<String, All<NonEmpty, All<Ascii, Trimmed>>>;
type Retries = Refined<U13, Positive>;

fn start(name: ServiceName, retries: Retries) {
    println!(
        "Starting {} with {} retries",
        name.get(),
        retries.get().get()
    );
}
fn main() {
    let name = ServiceName::new("worker".to_owned()).expect("valid service name");
    let retries = Retries::new(U13::new(3).unwrap()).expect("positive retry count");
    start(name, retries);
}
