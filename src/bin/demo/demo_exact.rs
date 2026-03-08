use symbolic_mr::{FermionOpKind, Index, IndexSpace, OperatorProduct, TensorTerm};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExactSystem {
    core: usize,
    active: usize,
    virtual_: usize,
}

impl Default for ExactSystem {
    fn default() -> Self {
        Self {
            core: 2,
            active: 3,
            virtual_: 2,
        }
    }
}

impl ExactSystem {
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
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExactIndexAssignment {
    entries: Vec<(IndexSpace, &'static str, usize)>,
}

impl ExactIndexAssignment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn active(mut self, symbol: &'static str, orbital: usize) -> Self {
        self.entries.push((IndexSpace::Active, symbol, orbital));
        self
    }

    pub fn virtual_(mut self, symbol: &'static str, orbital: usize) -> Self {
        self.entries.push((IndexSpace::Virtual, symbol, orbital));
        self
    }

    pub fn orbital(&self, index: &Index) -> usize {
        self.entries
            .iter()
            .find(|(space, symbol, _)| *space == index.space() && *symbol == index.symbol())
            .map(|(_, _, orbital)| *orbital)
            .unwrap_or_else(|| {
                panic!(
                    "missing exact assignment for {} in {:?}",
                    index.symbol(),
                    index.space()
                )
            })
    }

    pub fn delta_value(&self, left: &Index, right: &Index) -> f64 {
        if self.orbital(left) == self.orbital(right) {
            1.0
        } else {
            0.0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Determinant {
    occupancy: u16,
}

impl Determinant {
    pub fn empty() -> Self {
        Self { occupancy: 0 }
    }

    fn is_occupied(&self, orbital: usize) -> bool {
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
            (1u16 << orbital) - 1
        };
        (self.occupancy & mask).count_ones()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WeightedDeterminant {
    coefficient: f64,
    state: Determinant,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExactState {
    amplitudes: Vec<WeightedDeterminant>,
}

impl ExactState {
    pub fn demo_reference(system: &ExactSystem) -> Self {
        let base = base_core_determinant(system);
        let active_subsets = 1usize << system.active;
        let coefficient = 1.0 / (active_subsets as f64).sqrt();
        let mut amplitudes = Vec::with_capacity(active_subsets);

        for subset in 0..active_subsets {
            let mut state = base;
            for bit in 0..system.active {
                if ((subset >> bit) & 1) == 1 {
                    state = state.with_occupied(system.active_orbital(bit));
                }
            }
            amplitudes.push(WeightedDeterminant { coefficient, state });
        }

        Self { amplitudes }
    }
}

pub fn exact_expectation_for_product(
    state: &ExactState,
    assignment: &ExactIndexAssignment,
    product: &OperatorProduct,
) -> f64 {
    let ops = product
        .ops()
        .iter()
        .map(|op| (op.kind(), assignment.orbital(op.index())))
        .collect::<Vec<_>>();

    exact_expectation_from_ops(state, &ops)
}

pub fn evaluate_tensor_term(
    state: &ExactState,
    assignment: &ExactIndexAssignment,
    term: &TensorTerm,
) -> f64 {
    match term {
        TensorTerm::Zero => 0.0,
        TensorTerm::Delta(left, right) => assignment.delta_value(left, right),
        TensorTerm::Gamma(left, right) => exact_expectation_from_ops(
            state,
            &[
                (FermionOpKind::Create, assignment.orbital(left)),
                (FermionOpKind::Annihilate, assignment.orbital(right)),
            ],
        ),
        TensorTerm::Gamma2(left, right, lower_left, lower_right) => exact_expectation_from_ops(
            state,
            &[
                (FermionOpKind::Create, assignment.orbital(left)),
                (FermionOpKind::Create, assignment.orbital(right)),
                (FermionOpKind::Annihilate, assignment.orbital(lower_left)),
                (FermionOpKind::Annihilate, assignment.orbital(lower_right)),
            ],
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

fn exact_expectation_from_ops(state: &ExactState, ops: &[(FermionOpKind, usize)]) -> f64 {
    let mut total = 0.0;

    for bra in &state.amplitudes {
        for ket in &state.amplitudes {
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

fn base_core_determinant(system: &ExactSystem) -> Determinant {
    let mut det = Determinant::empty();
    for orbital in 0..system.core {
        det = det.with_occupied(system.core_orbital(orbital));
    }
    det
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

fn apply_create(det: Determinant, orbital: usize) -> Option<WeightedDeterminant> {
    if det.is_occupied(orbital) {
        return None;
    }

    Some(WeightedDeterminant {
        coefficient: fermionic_sign(det.occupied_below(orbital)),
        state: det.with_occupied(orbital),
    })
}

fn apply_annihilate(det: Determinant, orbital: usize) -> Option<WeightedDeterminant> {
    if !det.is_occupied(orbital) {
        return None;
    }

    Some(WeightedDeterminant {
        coefficient: fermionic_sign(det.occupied_below(orbital)),
        state: det.with_unoccupied(orbital),
    })
}

fn fermionic_sign(occupied_below: u32) -> f64 {
    if occupied_below.is_multiple_of(2) {
        1.0
    } else {
        -1.0
    }
}
