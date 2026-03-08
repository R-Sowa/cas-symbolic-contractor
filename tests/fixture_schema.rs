use symbolic_mr::fixtures::{
    FixtureCase, FixtureSuite, MatrixElementFixtureCase, MatrixElementFixtureSuite,
};

#[test]
fn serializes_and_deserializes_fixture_suite() {
    let suite = FixtureSuite {
        version: 1,
        suite: "reference_one_body".to_string(),
        cases: vec![FixtureCase {
            name: "active_to_gamma".to_string(),
            input: vec![
                "create(u:active)".to_string(),
                "annihilate(v:active)".to_string(),
            ],
            expected: "gamma(u,v)".to_string(),
        }],
    };

    let json = serde_json::to_string(&suite).unwrap();
    let decoded: FixtureSuite = serde_json::from_str(&json).unwrap();

    assert_eq!(decoded, suite);
}

#[test]
fn serializes_and_deserializes_matrix_element_fixture_suite() {
    let suite = MatrixElementFixtureSuite {
        version: 1,
        suite: "matrix_element".to_string(),
        cases: vec![MatrixElementFixtureCase {
            name: "canonical_overlap_like".to_string(),
            left: vec![
                "create(u:active)".to_string(),
                "annihilate(a:virtual)".to_string(),
            ],
            hamiltonian: None,
            right: vec![
                "create(b:virtual)".to_string(),
                "annihilate(v:active)".to_string(),
            ],
            expected: "delta(a,b) * gamma(u,v)".to_string(),
        }],
    };

    let json = serde_json::to_string(&suite).unwrap();
    let decoded: MatrixElementFixtureSuite = serde_json::from_str(&json).unwrap();

    assert_eq!(decoded, suite);
}
