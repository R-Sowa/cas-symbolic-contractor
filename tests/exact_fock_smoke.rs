mod support;

use support::exact_fock::{Determinant, SmallSystem, apply_annihilate, apply_create};

#[test]
fn create_on_empty_orbital_succeeds() {
    let system = SmallSystem::default();
    let det = Determinant::empty();
    let orbital = system.core_orbital(0);

    let after_create = apply_create(det, orbital).expect("creation should succeed");

    assert_eq!(after_create.coefficient, 1.0);
    assert!(after_create.state.is_occupied(orbital));
}

#[test]
fn annihilate_on_empty_orbital_returns_none() {
    let system = SmallSystem::default();
    let det = Determinant::empty();
    let orbital = system.core_orbital(0);

    let after_annihilate = apply_annihilate(det, orbital);

    assert!(after_annihilate.is_none());
}

#[test]
fn create_then_annihilate_restores_state() {
    let system = SmallSystem::default();
    let det = Determinant::empty();
    let orbital = system.active_orbital(1);

    let after_create = apply_create(det, orbital).expect("creation should succeed");
    let after_annihilate =
        apply_annihilate(after_create.state, orbital).expect("annihilation should succeed");

    assert_eq!(after_create.coefficient * after_annihilate.coefficient, 1.0);
    assert_eq!(after_annihilate.state, det);
}

#[test]
fn swapping_different_operators_flips_sign() {
    let system = SmallSystem::default();
    let det = Determinant::empty();
    let p = system.core_orbital(0);
    let q = system.active_orbital(0);

    let pq = apply_create(apply_create(det, p).unwrap().state, q).unwrap();
    let qp = apply_create(apply_create(det, q).unwrap().state, p).unwrap();

    assert_eq!(pq.state, qp.state);
    assert_eq!(pq.coefficient, -qp.coefficient);
}
