use symbolic_mr::{
    CasReference, active, annihilate, core, create, expectation,
    fixtures::{build_expectation_from_fixture_case, load_fixture_suite},
    simplify_tensor_form,
};

#[test]
fn reduces_active_two_body_expectation_to_gamma2() {
    let suite = load_fixture_suite("reference_two_body").unwrap();

    for case in suite.cases {
        let expr = build_expectation_from_fixture_case(&case, CasReference::new()).unwrap();
        let actual = simplify_tensor_form(expr, Default::default())
            .unwrap()
            .to_string();
        assert_eq!(actual, case.expected, "fixture case {}", case.name);
    }
}

#[test]
fn reduces_non_normal_ordered_two_body_input_via_normal_ordering() {
    let u = active("u");
    let v = active("v");
    let w = active("w");
    let x = active("x");

    let expr = expectation(
        annihilate(x) * create(u) * create(v) * annihilate(w),
        CasReference::new(),
    );

    let simplified = simplify_tensor_form(expr, Default::default()).unwrap();
    assert_eq!(
        simplified.to_string(),
        "delta(x,u) * gamma(v,w) - delta(x,v) * gamma(u,w) - Gamma(u,v;w,x)"
    );
}

#[test]
fn reduces_non_normal_ordered_core_active_two_body_input_with_sign() {
    let i = core("i");
    let j = core("j");
    let u = active("u");
    let v = active("v");

    let expr = expectation(
        create(u) * create(i) * annihilate(j) * annihilate(v),
        CasReference::new(),
    );

    let simplified = simplify_tensor_form(expr, Default::default()).unwrap();
    assert_eq!(simplified.to_string(), "delta(i,j) * gamma(u,v)");
}
