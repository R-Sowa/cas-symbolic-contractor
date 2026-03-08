mod support;

use proptest::prelude::*;
use support::exact_fock::{
    SmallSystem, evaluate_tensor_term, exact_expectation_for_product, fixed_index_assignment,
    fixed_reference_state,
};
use support::generators::{
    one_body_supported_strategy, two_body_active_only_strategy,
    two_body_core_active_mixed_strategy, unsupported_general_index_cases_strategy,
    unsupported_higher_body_cases_strategy, virtual_containing_zero_strategy,
};
use symbolic_mr::{
    CasReference, TensorSimplifyError, active, annihilate, create, expectation, general,
    operator_string, simplify_tensor_form,
};

fn supported_reference_strategy() -> BoxedStrategy<symbolic_mr::OperatorProduct> {
    prop_oneof![
        one_body_supported_strategy(),
        two_body_active_only_strategy(),
        two_body_core_active_mixed_strategy(),
    ]
    .boxed()
}

proptest! {
    #[test]
    fn supported_reference_reduction_matches_exact_expectation(expr in supported_reference_strategy()) {
        let system = SmallSystem::default();
        let assignment = fixed_index_assignment(&system);
        let state = fixed_reference_state(&system);

        let simplified = simplify_tensor_form(expectation(expr.clone(), CasReference::new()), Default::default()).unwrap();
        let exact = exact_expectation_for_product(&state, &assignment, &expr);
        let symbolic = evaluate_tensor_term(&state, &assignment, simplified.term());

        prop_assert!((exact - symbolic).abs() < 1.0e-9);
    }

    #[test]
    fn mathematically_zero_cases_remain_zero(expr in virtual_containing_zero_strategy()) {
        let system = SmallSystem::default();
        let assignment = fixed_index_assignment(&system);
        let state = fixed_reference_state(&system);

        let simplified = simplify_tensor_form(expectation(expr.clone(), CasReference::new()), Default::default()).unwrap();
        let exact = exact_expectation_for_product(&state, &assignment, &expr);
        let symbolic = evaluate_tensor_term(&state, &assignment, simplified.term());

        prop_assert!(exact.abs() < 1.0e-9);
        prop_assert!(symbolic.abs() < 1.0e-9);
        prop_assert_eq!(simplified.to_string(), "0");
    }

    #[test]
    fn unsupported_general_cases_return_error(expr in unsupported_general_index_cases_strategy()) {
        let result = simplify_tensor_form(expectation(expr, CasReference::new()), Default::default());

        prop_assert!(matches!(result, Err(TensorSimplifyError::UnsupportedReferenceCase)));
    }

    #[test]
    fn unsupported_higher_body_cases_return_error(expr in unsupported_higher_body_cases_strategy()) {
        let result = simplify_tensor_form(expectation(expr, CasReference::new()), Default::default());

        prop_assert!(matches!(result, Err(TensorSimplifyError::UnsupportedReferenceCase)));
    }
}

#[test]
fn general_diagonal_one_body_is_not_silently_zeroed() {
    let system = SmallSystem::default();
    let assignment = fixed_index_assignment(&system);
    let state = fixed_reference_state(&system);
    let expr = create(general("p")) * annihilate(general("p"));

    let exact = exact_expectation_for_product(&state, &assignment, &expr);
    assert!(exact.abs() > 1.0e-9);

    let result = simplify_tensor_form(expectation(expr, CasReference::new()), Default::default());
    assert!(matches!(
        result,
        Err(TensorSimplifyError::UnsupportedReferenceCase)
    ));
}

#[test]
fn active_three_body_structure_survives_as_higher_rdm() {
    let system = SmallSystem::default();
    let assignment = fixed_index_assignment(&system);
    let state = fixed_reference_state(&system);
    let expr = operator_string(vec![
        create(active("u")),
        create(active("v")),
        create(active("u")),
        annihilate(active("u")),
        annihilate(active("v")),
        annihilate(active("u")),
    ]);

    let simplified = simplify_tensor_form(
        expectation(expr.clone(), CasReference::new()),
        Default::default(),
    )
    .unwrap();
    let exact = exact_expectation_for_product(&state, &assignment, &expr);
    let symbolic = evaluate_tensor_term(&state, &assignment, simplified.term());

    assert!((exact - symbolic).abs() < 1.0e-9);
}
