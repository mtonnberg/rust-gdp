use rust_gdp::{predicates::Positive, truth, with_name, Refined};
#[no_mangle]
pub fn baseline(x: u64) -> u64 {
    x
}
#[no_mangle]
pub fn scoped(x: u64) -> u64 {
    with_name(x, |n| n.into_inner())
}
#[no_mangle]
pub fn refined(x: Refined<u64, Positive>) -> u64 {
    x.into_inner()
}
#[no_mangle]
pub fn logical(x: u64) -> u64 {
    let _ = truth().and(truth()).split();
    x
}
#[no_mangle]
pub fn evidence_roundtrip(x: Refined<u64, Positive>) -> u64 {
    x.with_name(|n, p| n.refine(p).into_inner())
}
#[no_mangle]
pub fn lexical(x: u64) -> u64 {
    rust_gdp::name!(value = x);
    value.into_inner()
}
#[no_mangle]
pub fn branded<'n>(
    x: rust_gdp::Named<'n, u64>,
    _: rust_gdp::Proof<rust_gdp::And<rust_gdp::Positive<'n>, rust_gdp::Even<'n>>>,
) -> u64 {
    x.into_inner()
}
