use symbolic_mr::{CasReference, active, annihilate, create, expectation, simplify_tensor_form};

#[test]
fn simplification_is_deterministic_and_idempotent() {
    let u = active("u");
    let v = active("v");
    let expr = expectation(create(u.clone()) * annihilate(v), CasReference::new());

    let once = simplify_tensor_form(expr.clone(), Default::default())
        .unwrap()
        .to_string();
    let twice = simplify_tensor_form(expr, Default::default())
        .unwrap()
        .to_string();

    assert_eq!(once, twice);
}
