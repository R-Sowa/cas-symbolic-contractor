use symbolic_mr::fixtures::{build_matrix_element_suite, build_reference_one_body_suite};

#[test]
fn fixture_builder_produces_expected_suite_name() {
    let suite = build_reference_one_body_suite();
    assert_eq!(suite.suite, "reference_one_body");
}

#[test]
fn matrix_element_fixture_builder_produces_expected_suite_name() {
    let suite = build_matrix_element_suite();
    assert_eq!(suite.suite, "matrix_element");
}

#[test]
fn matrix_element_fixture_builder_includes_hamiltonian_case() {
    let suite = build_matrix_element_suite();
    let case = suite
        .cases
        .iter()
        .find(|case| case.name == "one_body_hamiltonian_in_middle")
        .expect("missing Hamiltonian fixture case");

    assert!(case.hamiltonian.is_some());
    assert_eq!(case.expected, "- delta(a,b) * Gamma(u,x;v,y)");
}
