#![allow(dead_code)]

use std::collections::BTreeMap;

use symbolic_mr::{
    DeltaConstraint, FermionOpKind, Index, IndexSpace, NormalOrderedExpr, OperatorProduct,
    TensorTerm,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SmallSystem {
    core: usize,
    active: usize,
    virtual_: usize,
}

impl Default for SmallSystem {
    fn default() -> Self {
        Self {
            core: 2,
            active: 2,
            virtual_: 2,
        }
    }
}

impl SmallSystem {
    pub fn core_orbital(&self, index: usize) -> usize {
        assert!(index < self.core);
        index
    }

    pub fn active_orbital(&self, index: usize) -> usize {
        assert!(index < self.active);
        self.core + index
    }

    pub fn virtual_orbital(&self, index: usize) -> usize {
        assert!(index < self.virtual_);
        self.core + self.active + index
    }

    pub fn orbital_count(&self) -> usize {
        self.core + self.active + self.virtual_
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexAssignment {
    core: [usize; 2],
    active: [usize; 2],
    virtual_: [usize; 2],
    general: [usize; 4],
}

impl IndexAssignment {
    fn orbital(&self, index: &Index) -> usize {
        match (index.space(), index.symbol()) {
            (IndexSpace::Core, "i") => self.core[0],
            (IndexSpace::Core, "j") => self.core[1],
            (IndexSpace::Active, "u") => self.active[0],
            (IndexSpace::Active, "v") => self.active[1],
            (IndexSpace::Virtual, "a") => self.virtual_[0],
            (IndexSpace::Virtual, "b") => self.virtual_[1],
            (IndexSpace::General, "p") => self.general[0],
            (IndexSpace::General, "q") => self.general[1],
            (IndexSpace::General, "r") => self.general[2],
            (IndexSpace::General, "s") => self.general[3],
            _ => panic!(
                "unsupported test index assignment: {} in {:?}",
                index.symbol(),
                index.space()
            ),
        }
    }

    fn delta_value(&self, delta: &DeltaConstraint) -> f64 {
        if self.orbital(delta.left()) == self.orbital(delta.right()) {
            1.0
        } else {
            0.0
        }
    }

    fn delta_value_for_indices(&self, left: &Index, right: &Index) -> f64 {
        if self.orbital(left) == self.orbital(right) {
            1.0
        } else {
            0.0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Determinant {
    occupancy: u8,
}

impl Determinant {
    pub fn empty() -> Self {
        Self { occupancy: 0 }
    }

    pub fn is_occupied(&self, orbital: usize) -> bool {
        ((self.occupancy >> orbital) & 1) == 1
    }

    fn with_occupied(mut self, orbital: usize) -> Self {
        self.occupancy |= 1 << orbital;
        self
    }

    fn with_unoccupied(mut self, orbital: usize) -> Self {
        self.occupancy &= !(1 << orbital);
        self
    }

    fn occupied_below(&self, orbital: usize) -> u32 {
        let mask = if orbital == 0 {
            0
        } else {
            (1u8 << orbital) - 1
        };
        (self.occupancy & mask).count_ones()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WeightedDeterminant {
    pub coefficient: f64,
    pub state: Determinant,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExactState {
    amplitudes: Vec<WeightedDeterminant>,
}

impl ExactState {
    pub fn norm(&self) -> f64 {
        self.amplitudes
            .iter()
            .map(|term| term.coefficient * term.coefficient)
            .sum()
    }

    pub fn amplitudes(&self) -> &[WeightedDeterminant] {
        &self.amplitudes
    }
}

pub fn fixed_index_assignment(system: &SmallSystem) -> IndexAssignment {
    IndexAssignment {
        core: [system.core_orbital(0), system.core_orbital(1)],
        active: [system.active_orbital(0), system.active_orbital(1)],
        virtual_: [system.virtual_orbital(0), system.virtual_orbital(1)],
        general: [
            system.core_orbital(0),
            system.core_orbital(1),
            system.active_orbital(0),
            system.active_orbital(1),
        ],
    }
}

pub fn fixed_reference_state(system: &SmallSystem) -> ExactState {
    let base = base_core_determinant(system);
    let u = system.active_orbital(0);
    let v = system.active_orbital(1);

    ExactState {
        amplitudes: vec![
            WeightedDeterminant {
                coefficient: 0.5,
                state: base,
            },
            WeightedDeterminant {
                coefficient: 0.5,
                state: base.with_occupied(u),
            },
            WeightedDeterminant {
                coefficient: 0.5,
                state: base.with_occupied(v),
            },
            WeightedDeterminant {
                coefficient: 0.5,
                state: base.with_occupied(u).with_occupied(v),
            },
        ],
    }
}

pub fn apply_create(det: Determinant, orbital: usize) -> Option<WeightedDeterminant> {
    if det.is_occupied(orbital) {
        return None;
    }

    let sign = fermionic_sign(det.occupied_below(orbital));
    Some(WeightedDeterminant {
        coefficient: sign,
        state: det.with_occupied(orbital),
    })
}

pub fn apply_annihilate(det: Determinant, orbital: usize) -> Option<WeightedDeterminant> {
    if !det.is_occupied(orbital) {
        return None;
    }

    let sign = fermionic_sign(det.occupied_below(orbital));
    Some(WeightedDeterminant {
        coefficient: sign,
        state: det.with_unoccupied(orbital),
    })
}

pub fn exact_expectation_from_ops(state: &ExactState, ops: &[(FermionOpKind, usize)]) -> f64 {
    let mut total = 0.0;

    for bra in state.amplitudes() {
        for ket in state.amplitudes() {
            let Some(image) = apply_operator_string(ket.state, ops) else {
                continue;
            };

            if image.state == bra.state {
                total += bra.coefficient * ket.coefficient * image.coefficient;
            }
        }
    }

    total
}

pub fn exact_expectation_for_product(
    state: &ExactState,
    assignment: &IndexAssignment,
    product: &OperatorProduct,
) -> f64 {
    let ops = product
        .ops()
        .iter()
        .map(|op| (op.kind(), assignment.orbital(op.index())))
        .collect::<Vec<_>>();

    exact_expectation_from_ops(state, &ops)
}

pub fn exact_gamma(state: &ExactState, left: usize, right: usize) -> f64 {
    exact_expectation_from_ops(
        state,
        &[
            (FermionOpKind::Create, left),
            (FermionOpKind::Annihilate, right),
        ],
    )
}

pub fn exact_gamma2(
    state: &ExactState,
    left: usize,
    right: usize,
    lower_left: usize,
    lower_right: usize,
) -> f64 {
    exact_expectation_from_ops(
        state,
        &[
            (FermionOpKind::Create, left),
            (FermionOpKind::Create, right),
            (FermionOpKind::Annihilate, lower_left),
            (FermionOpKind::Annihilate, lower_right),
        ],
    )
}

pub fn evaluate_tensor_term(
    state: &ExactState,
    assignment: &IndexAssignment,
    term: &TensorTerm,
) -> f64 {
    match term {
        TensorTerm::Zero => 0.0,
        TensorTerm::Delta(left, right) => assignment.delta_value_for_indices(left, right),
        TensorTerm::Gamma(left, right) => {
            exact_gamma(state, assignment.orbital(left), assignment.orbital(right))
        }
        TensorTerm::Gamma2(left, right, lower_left, lower_right) => exact_gamma2(
            state,
            assignment.orbital(left),
            assignment.orbital(right),
            assignment.orbital(lower_left),
            assignment.orbital(lower_right),
        ),
        TensorTerm::Product(factors) => factors
            .iter()
            .map(|factor| evaluate_tensor_term(state, assignment, factor))
            .product(),
        TensorTerm::Sum(terms) => terms
            .iter()
            .map(|signed| {
                f64::from(signed.coefficient())
                    * evaluate_tensor_term(state, assignment, signed.term())
            })
            .sum(),
        TensorTerm::HigherRdm { product, .. } => {
            exact_expectation_for_product(state, assignment, product)
        }
    }
}

pub fn normal_order_actions_match_on_basis(
    system: &SmallSystem,
    assignment: &IndexAssignment,
    product: &OperatorProduct,
    ordered: &NormalOrderedExpr,
) -> bool {
    basis_determinants(system)
        .into_iter()
        .all(|det| action_matches(det, assignment, product, ordered))
}

fn base_core_determinant(system: &SmallSystem) -> Determinant {
    let mut det = Determinant::empty();
    for orbital in 0..system.core {
        det = det.with_occupied(system.core_orbital(orbital));
    }
    det
}

fn basis_determinants(system: &SmallSystem) -> Vec<Determinant> {
    (0..(1usize << system.orbital_count()))
        .map(|occupancy| Determinant {
            occupancy: occupancy as u8,
        })
        .collect()
}

fn action_matches(
    det: Determinant,
    assignment: &IndexAssignment,
    product: &OperatorProduct,
    ordered: &NormalOrderedExpr,
) -> bool {
    let original = evaluate_product_action(det, assignment, product);
    let rewritten = evaluate_normal_ordered_action(det, assignment, ordered);

    original == rewritten
}

fn apply_operator_string(
    det: Determinant,
    ops: &[(FermionOpKind, usize)],
) -> Option<WeightedDeterminant> {
    let mut coefficient = 1.0;
    let mut state = det;

    for &(kind, orbital) in ops.iter().rev() {
        let step = match kind {
            FermionOpKind::Create => apply_create(state, orbital),
            FermionOpKind::Annihilate => apply_annihilate(state, orbital),
        }?;
        coefficient *= step.coefficient;
        state = step.state;
    }

    Some(WeightedDeterminant { coefficient, state })
}

fn evaluate_product_action(
    det: Determinant,
    assignment: &IndexAssignment,
    product: &OperatorProduct,
) -> BTreeMap<u8, i32> {
    let ops = product
        .ops()
        .iter()
        .map(|op| (op.kind(), assignment.orbital(op.index())))
        .collect::<Vec<_>>();

    evaluate_operator_string(det, &ops, 1)
}

fn evaluate_normal_ordered_action(
    det: Determinant,
    assignment: &IndexAssignment,
    expr: &NormalOrderedExpr,
) -> BTreeMap<u8, i32> {
    let mut total = BTreeMap::new();

    for term in expr.terms() {
        let delta_factor = term
            .deltas()
            .iter()
            .map(|delta| assignment.delta_value(delta) as i32)
            .product::<i32>();
        if delta_factor == 0 {
            continue;
        }

        let ops = term
            .product()
            .ops()
            .iter()
            .map(|op| (op.kind(), assignment.orbital(op.index())))
            .collect::<Vec<_>>();
        let coefficient = i32::from(term.coefficient()) * delta_factor;
        let image = evaluate_operator_string(det, &ops, coefficient);

        for (occupancy, amplitude) in image {
            *total.entry(occupancy).or_insert(0) += amplitude;
        }
    }

    total.retain(|_, amplitude| *amplitude != 0);
    total
}

fn evaluate_operator_string(
    det: Determinant,
    ops: &[(FermionOpKind, usize)],
    coefficient: i32,
) -> BTreeMap<u8, i32> {
    let mut total = BTreeMap::new();

    let Some(image) = apply_operator_string(det, ops) else {
        return total;
    };

    let amplitude = coefficient * (image.coefficient as i32);
    if amplitude != 0 {
        total.insert(image.state.occupancy, amplitude);
    }

    total
}

fn fermionic_sign(occupied_below: u32) -> f64 {
    if occupied_below.is_multiple_of(2) {
        1.0
    } else {
        -1.0
    }
}
