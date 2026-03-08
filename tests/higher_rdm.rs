use symbolic_mr::{CasReference, active, annihilate, create, expectation, simplify_tensor_form};

#[test]
fn marks_terms_that_need_higher_rdms() {
    let u = active("u");
    let v = active("v");
    let w = active("w");
    let x = active("x");
    let y = active("y");
    let z = active("z");

    let expr = expectation(
        create(u.clone())
            * create(v.clone())
            * create(w.clone())
            * annihilate(z.clone())
            * annihilate(y.clone())
            * annihilate(x.clone()),
        CasReference::new(),
    );

    let simplified = simplify_tensor_form(expr, Default::default()).unwrap();
    assert!(simplified.to_string().contains("HigherRDM"));
}
