use crate::ast::{FermionOp, FermionOpKind, Index, IndexSpace, OperatorProduct};
use crate::constraints::DeltaConstraint;
use crate::reference::{CasReference, Expectation, MatrixElement};

pub fn crate_name() -> &'static str {
    "symbolic_mr"
}

pub fn core(symbol: impl Into<String>) -> Index {
    Index::new(symbol, IndexSpace::Core)
}

pub fn active(symbol: impl Into<String>) -> Index {
    Index::new(symbol, IndexSpace::Active)
}

pub fn virtual_(symbol: impl Into<String>) -> Index {
    Index::new(symbol, IndexSpace::Virtual)
}

pub fn general(symbol: impl Into<String>) -> Index {
    Index::new(symbol, IndexSpace::General)
}

pub fn create(index: Index) -> FermionOp {
    FermionOp::new(FermionOpKind::Create, index)
}

pub fn annihilate(index: Index) -> FermionOp {
    FermionOp::new(FermionOpKind::Annihilate, index)
}

pub fn expectation(product: OperatorProduct, reference: CasReference) -> Expectation {
    Expectation::new(product, reference)
}

pub fn operator_string(ops: Vec<FermionOp>) -> OperatorProduct {
    OperatorProduct::new(ops)
}

pub fn delta(left: Index, right: Index) -> DeltaConstraint {
    DeltaConstraint::new(left, right)
}

pub fn matrix_element(
    left: OperatorProduct,
    hamiltonian: Option<OperatorProduct>,
    right: OperatorProduct,
    reference: CasReference,
) -> MatrixElement {
    MatrixElement::new(left, hamiltonian, right, reference)
}
