use symbolic_mr::{
    CasReference, active, annihilate, core, create, expectation,
    fixtures::{build_expectation_from_fixture_case, load_fixture_suite},
    simplify_tensor_form,
};

#[test]
fn reduces_one_body_expectations_to_gamma_or_zero() {
    let suite = load_fixture_suite("reference_one_body").unwrap();

    for case in suite.cases {
        let expr = build_expectation_from_fixture_case(&case, CasReference::new()).unwrap();
        let actual = simplify_tensor_form(expr, Default::default())
            .unwrap()
            .to_string();
        assert_eq!(actual, case.expected, "fixture case {}", case.name);
    }
}

#[test]
fn does_not_treat_non_normal_ordered_two_op_input_as_gamma() {
    let u = active("u");
    let v = active("v");
    let expr = expectation(annihilate(v) * create(u), CasReference::new());

    let simplified = simplify_tensor_form(expr, Default::default()).unwrap();
    assert_eq!(simplified.to_string(), "delta(v,u) - gamma(u,v)");
}

#[test]
fn reduces_non_normal_core_core_input_via_normal_ordering() {
    let i = core("i");
    let j = core("j");
    let expr = expectation(annihilate(j) * create(i), CasReference::new());

    let simplified = simplify_tensor_form(expr, Default::default()).unwrap();
    assert_eq!(simplified.to_string(), "delta(j,i) - delta(i,j)");
}
