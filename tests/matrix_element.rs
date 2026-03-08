use symbolic_mr::{
    CasReference,
    fixtures::{build_matrix_element_from_fixture_case, load_matrix_element_fixture_suite},
    simplify_tensor_form,
};

#[test]
fn reduces_matrix_element_cases_from_fixtures() {
    let suite = load_matrix_element_fixture_suite("matrix_element").unwrap();

    for case in suite.cases {
        let expr = build_matrix_element_from_fixture_case(&case, CasReference::new()).unwrap();
        let actual = simplify_tensor_form(expr, Default::default())
            .unwrap()
            .to_string();
        assert_eq!(actual, case.expected, "fixture case {}", case.name);
    }
}
