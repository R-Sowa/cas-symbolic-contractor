mod support;

use proptest::strategy::{Strategy, ValueTree};
use support::generators::{
    non_normal_ordered_cases_strategy, one_body_supported_strategy, repeated_index_cases_strategy,
    two_body_active_only_strategy, unsupported_general_index_cases_strategy,
};

#[test]
fn one_body_supported_generator_emits_two_ops() {
    let mut runner = proptest::test_runner::TestRunner::default();
    let expr = one_body_supported_strategy()
        .new_tree(&mut runner)
        .unwrap()
        .current();

    assert_eq!(expr.ops().len(), 2);
}

#[test]
fn active_only_two_body_generator_emits_four_active_ops() {
    let mut runner = proptest::test_runner::TestRunner::default();
    let expr = two_body_active_only_strategy()
        .new_tree(&mut runner)
        .unwrap()
        .current();

    assert_eq!(expr.ops().len(), 4);
    assert!(
        expr.ops()
            .iter()
            .all(|op: &symbolic_mr::FermionOp| matches!(
                op.index().space(),
                symbolic_mr::IndexSpace::Active
            ))
    );
}

#[test]
fn unsupported_general_generator_emits_general_index() {
    let mut runner = proptest::test_runner::TestRunner::default();
    let expr = unsupported_general_index_cases_strategy()
        .new_tree(&mut runner)
        .unwrap()
        .current();

    assert!(
        expr.ops()
            .iter()
            .any(|op: &symbolic_mr::FermionOp| matches!(
                op.index().space(),
                symbolic_mr::IndexSpace::General
            ))
    );
}

#[test]
fn repeated_index_generator_reuses_a_symbol() {
    let mut runner = proptest::test_runner::TestRunner::default();
    let expr = repeated_index_cases_strategy()
        .new_tree(&mut runner)
        .unwrap()
        .current();

    let mut symbols = expr
        .ops()
        .iter()
        .map(|op: &symbolic_mr::FermionOp| op.index().symbol().to_string())
        .collect::<Vec<_>>();
    symbols.sort();
    symbols.dedup();

    assert!(symbols.len() < expr.ops().len());
}

#[test]
fn non_normal_generator_requires_a_rewrite() {
    let mut runner = proptest::test_runner::TestRunner::default();
    let expr = non_normal_ordered_cases_strategy()
        .new_tree(&mut runner)
        .unwrap()
        .current();
    let original = expr.to_string();
    let ordered = symbolic_mr::normal_order_product(expr).unwrap().to_string();

    assert_ne!(original, ordered);
}
