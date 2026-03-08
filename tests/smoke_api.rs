use symbolic_mr::{CasReference, active, annihilate, create, expectation};

#[test]
fn smoke_builds_a_simple_expectation() {
    let u = active("u");
    let v = active("v");
    let expr = expectation(
        create(u.clone()) * annihilate(v.clone()),
        CasReference::new(),
    );
    let rendered = format!("{expr:?}");
    assert!(rendered.contains("Expectation"));
}
