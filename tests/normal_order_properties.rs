mod support;

use proptest::prelude::*;
use support::exact_fock::{
    SmallSystem, fixed_index_assignment, normal_order_actions_match_on_basis,
};
use support::generators::{
    non_normal_ordered_cases_strategy, one_body_supported_strategy, repeated_index_cases_strategy,
    two_body_active_only_strategy, unsupported_general_index_cases_strategy,
};
use symbolic_mr::{FermionOpKind, Index, IndexSpace, NormalOrderedTerm, OperatorProduct};

fn operator_expression_strategy() -> BoxedStrategy<OperatorProduct> {
    prop_oneof![
        one_body_supported_strategy(),
        two_body_active_only_strategy(),
        non_normal_ordered_cases_strategy(),
        repeated_index_cases_strategy(),
        unsupported_general_index_cases_strategy(),
    ]
    .boxed()
}

proptest! {
    #[test]
    fn normal_order_output_terms_are_normal(expr in operator_expression_strategy()) {
        let ordered = symbolic_mr::normal_order_product(expr).unwrap();

        prop_assert!(ordered.terms().iter().all(is_normal_ordered_term));
    }

    #[test]
    fn normal_order_output_terms_are_idempotent(expr in operator_expression_strategy()) {
        let ordered = symbolic_mr::normal_order_product(expr).unwrap();

        for term in ordered.terms() {
            if term.product().ops().is_empty() {
                continue;
            }

            let rerun = symbolic_mr::normal_order_product(term.product().clone()).unwrap();
            prop_assert_eq!(rerun.terms().len(), 1);
            prop_assert_eq!(rerun.terms()[0].coefficient(), 1);
            prop_assert!(rerun.terms()[0].deltas().is_empty());
            prop_assert_eq!(rerun.terms()[0].product(), term.product());
        }
    }

    #[test]
    fn normal_ordering_preserves_operator_action(expr in operator_expression_strategy()) {
        let system = SmallSystem::default();
        let assignment = fixed_index_assignment(&system);
        let ordered = symbolic_mr::normal_order_product(expr.clone()).unwrap();

        prop_assert!(normal_order_actions_match_on_basis(
            &system,
            &assignment,
            &expr,
            &ordered,
        ));
    }
}

fn is_normal_ordered_term(term: &NormalOrderedTerm) -> bool {
    let ops = term.product().ops();

    ops.windows(2)
        .all(|pair| match (pair[0].kind(), pair[1].kind()) {
            (FermionOpKind::Annihilate, FermionOpKind::Create) => false,
            (left, right) if left == right => {
                !compare_index(pair[0].index(), pair[1].index()).is_gt()
            }
            _ => true,
        })
}

fn compare_index(left: &Index, right: &Index) -> std::cmp::Ordering {
    left.symbol()
        .cmp(right.symbol())
        .then_with(|| index_space_rank(left.space()).cmp(&index_space_rank(right.space())))
}

fn index_space_rank(space: IndexSpace) -> u8 {
    match space {
        IndexSpace::Core => 0,
        IndexSpace::Active => 1,
        IndexSpace::Virtual => 2,
        IndexSpace::General => 3,
    }
}
