mod support;

use support::exact_fock::{
    SmallSystem, exact_expectation_from_ops, exact_gamma, exact_gamma2, fixed_reference_state,
};
use symbolic_mr::FermionOpKind;

#[test]
fn fixed_reference_state_is_normalized() {
    let system = SmallSystem::default();
    let reference = fixed_reference_state(&system);

    let norm = reference.norm();

    assert!((norm - 1.0).abs() < 1.0e-12);
}

#[test]
fn exact_gamma_matches_direct_one_body_expectation() {
    let system = SmallSystem::default();
    let reference = fixed_reference_state(&system);
    let u = system.active_orbital(0);
    let v = system.active_orbital(1);

    let direct = exact_expectation_from_ops(
        &reference,
        &[(FermionOpKind::Create, u), (FermionOpKind::Annihilate, v)],
    );

    assert!((exact_gamma(&reference, u, v) - direct).abs() < 1.0e-12);
}

#[test]
fn exact_gamma2_matches_direct_two_body_expectation() {
    let system = SmallSystem::default();
    let reference = fixed_reference_state(&system);
    let u = system.active_orbital(0);
    let v = system.active_orbital(1);

    let direct = exact_expectation_from_ops(
        &reference,
        &[
            (FermionOpKind::Create, u),
            (FermionOpKind::Create, v),
            (FermionOpKind::Annihilate, u),
            (FermionOpKind::Annihilate, v),
        ],
    );

    assert!((exact_gamma2(&reference, u, v, u, v) - direct).abs() < 1.0e-12);
}
