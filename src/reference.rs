use std::fmt;

use crate::ast::{FermionOpKind, Index, IndexSpace, OperatorProduct};
use crate::constraints::DeltaConstraint;
use crate::rewrite::{NormalOrderedTerm, SimplifyConfig, normal_order_product};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CasReference;

impl CasReference {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expectation {
    product: OperatorProduct,
    reference: CasReference,
}

impl Expectation {
    pub fn new(product: OperatorProduct, reference: CasReference) -> Self {
        Self { product, reference }
    }

    pub fn product(&self) -> &OperatorProduct {
        &self.product
    }

    pub fn reference(&self) -> CasReference {
        self.reference
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatrixElement {
    left: OperatorProduct,
    hamiltonian: Option<OperatorProduct>,
    right: OperatorProduct,
    reference: CasReference,
}

impl MatrixElement {
    pub fn new(
        left: OperatorProduct,
        hamiltonian: Option<OperatorProduct>,
        right: OperatorProduct,
        reference: CasReference,
    ) -> Self {
        Self {
            left,
            hamiltonian,
            right,
            reference,
        }
    }

    pub fn left(&self) -> &OperatorProduct {
        &self.left
    }

    pub fn hamiltonian(&self) -> Option<&OperatorProduct> {
        self.hamiltonian.as_ref()
    }

    pub fn right(&self) -> &OperatorProduct {
        &self.right
    }

    pub fn reference(&self) -> CasReference {
        self.reference
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TensorSimplifyError {
    UnsupportedOperatorRank,
    UnsupportedReferenceCase,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TensorTerm {
    Zero,
    Delta(Index, Index),
    Gamma(Index, Index),
    Gamma2(Index, Index, Index, Index),
    Product(Vec<TensorTerm>),
    Sum(Vec<SignedTensorTerm>),
    HigherRdm {
        min_order: usize,
        product: OperatorProduct,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedTensorTerm {
    coefficient: i8,
    term: TensorTerm,
}

impl SignedTensorTerm {
    pub fn new(coefficient: i8, term: TensorTerm) -> Self {
        Self { coefficient, term }
    }

    pub fn coefficient(&self) -> i8 {
        self.coefficient
    }

    pub fn term(&self) -> &TensorTerm {
        &self.term
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimplifiedTensorForm {
    term: TensorTerm,
}

impl SimplifiedTensorForm {
    pub fn new(term: TensorTerm) -> Self {
        Self { term }
    }

    pub fn term(&self) -> &TensorTerm {
        &self.term
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TensorReductionTrace {
    normal_ordered: crate::rewrite::NormalOrderedExpr,
    reduced_terms: Vec<SignedTensorTerm>,
    final_form: SimplifiedTensorForm,
}

impl TensorReductionTrace {
    pub fn new(
        normal_ordered: crate::rewrite::NormalOrderedExpr,
        reduced_terms: Vec<SignedTensorTerm>,
        final_form: SimplifiedTensorForm,
    ) -> Self {
        Self {
            normal_ordered,
            reduced_terms,
            final_form,
        }
    }

    pub fn normal_ordered(&self) -> &crate::rewrite::NormalOrderedExpr {
        &self.normal_ordered
    }

    pub fn reduced_terms(&self) -> &[SignedTensorTerm] {
        &self.reduced_terms
    }

    pub fn final_form(&self) -> &SimplifiedTensorForm {
        &self.final_form
    }
}

pub enum TensorInput {
    Expectation(Expectation),
    Delta(DeltaConstraint),
    MatrixElement(MatrixElement),
}

impl From<Expectation> for TensorInput {
    fn from(value: Expectation) -> Self {
        Self::Expectation(value)
    }
}

impl From<DeltaConstraint> for TensorInput {
    fn from(value: DeltaConstraint) -> Self {
        Self::Delta(value)
    }
}

impl From<MatrixElement> for TensorInput {
    fn from(value: MatrixElement) -> Self {
        Self::MatrixElement(value)
    }
}

pub fn simplify_tensor_form<T: Into<TensorInput>>(
    input: T,
    _config: SimplifyConfig,
) -> Result<SimplifiedTensorForm, TensorSimplifyError> {
    match input.into() {
        TensorInput::Expectation(expectation) => simplify_expectation(expectation),
        TensorInput::Delta(delta) => simplify_delta(delta),
        TensorInput::MatrixElement(matrix_element) => simplify_matrix_element(matrix_element),
    }
}

pub fn trace_expectation(
    expectation: Expectation,
    _config: SimplifyConfig,
) -> Result<TensorReductionTrace, TensorSimplifyError> {
    trace_product_in_reference(expectation.product().clone())
}

pub fn trace_matrix_element(
    matrix_element: MatrixElement,
    _config: SimplifyConfig,
) -> Result<TensorReductionTrace, TensorSimplifyError> {
    let mut ops = matrix_element.left().ops().to_vec();
    if let Some(hamiltonian) = matrix_element.hamiltonian() {
        ops.extend_from_slice(hamiltonian.ops());
    }
    ops.extend_from_slice(matrix_element.right().ops());

    trace_product_in_reference(OperatorProduct::new(ops))
}

fn simplify_delta(delta: DeltaConstraint) -> Result<SimplifiedTensorForm, TensorSimplifyError> {
    if delta.is_contradictory() {
        return Ok(SimplifiedTensorForm::new(TensorTerm::Zero));
    }

    Ok(SimplifiedTensorForm::new(TensorTerm::Delta(
        delta.left().clone(),
        delta.right().clone(),
    )))
}

fn simplify_expectation(
    expectation: Expectation,
) -> Result<SimplifiedTensorForm, TensorSimplifyError> {
    simplify_product_in_reference(expectation.product().clone())
}

fn simplify_product_in_reference(
    product: OperatorProduct,
) -> Result<SimplifiedTensorForm, TensorSimplifyError> {
    Ok(trace_product_in_reference(product)?.final_form)
}

fn trace_product_in_reference(
    product: OperatorProduct,
) -> Result<TensorReductionTrace, TensorSimplifyError> {
    let ordered =
        normal_order_product(product).map_err(|_| TensorSimplifyError::UnsupportedReferenceCase)?;
    let reduced_terms = ordered
        .terms()
        .iter()
        .map(reduce_reference_term)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    let final_form = collapse_signed_terms(reduced_terms.clone())?;

    Ok(TensorReductionTrace::new(
        ordered,
        reduced_terms,
        final_form,
    ))
}

fn simplify_matrix_element(
    matrix_element: MatrixElement,
) -> Result<SimplifiedTensorForm, TensorSimplifyError> {
    let mut ops = matrix_element.left().ops().to_vec();
    if let Some(hamiltonian) = matrix_element.hamiltonian() {
        ops.extend_from_slice(hamiltonian.ops());
    }
    ops.extend_from_slice(matrix_element.right().ops());

    simplify_product_in_reference(OperatorProduct::new(ops))
}

impl fmt::Display for SimplifiedTensorForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.term)
    }
}

impl fmt::Display for SignedTensorTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let is_negative = self.coefficient < 0;
        let abs = self.coefficient.unsigned_abs();

        if is_negative {
            write!(f, "- ")?;
        }

        if abs != 1 {
            write!(f, "{abs} ")?;
        }

        write!(f, "{}", self.term)
    }
}

impl fmt::Display for TensorTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TensorTerm::Zero => write!(f, "0"),
            TensorTerm::Delta(left, right) => {
                write!(f, "delta({},{})", left.symbol(), right.symbol())
            }
            TensorTerm::Gamma(left, right) => {
                write!(f, "gamma({},{})", left.symbol(), right.symbol())
            }
            TensorTerm::Gamma2(left, right, lower_left, lower_right) => write!(
                f,
                "Gamma({},{};{},{})",
                left.symbol(),
                right.symbol(),
                lower_left.symbol(),
                lower_right.symbol()
            ),
            TensorTerm::Product(factors) => {
                for (idx, factor) in factors.iter().enumerate() {
                    if idx > 0 {
                        write!(f, " * ")?;
                    }
                    write!(f, "{factor}")?;
                }
                Ok(())
            }
            TensorTerm::Sum(terms) => {
                for (idx, signed) in terms.iter().enumerate() {
                    let is_negative = signed.coefficient < 0;
                    if idx == 0 {
                        if is_negative {
                            write!(f, "- ")?;
                        }
                    } else if is_negative {
                        write!(f, " - ")?;
                    } else {
                        write!(f, " + ")?;
                    }

                    let abs = signed.coefficient.unsigned_abs();
                    if abs != 1 {
                        write!(f, "{abs} ")?;
                    }

                    write!(f, "{}", signed.term)?;
                }
                Ok(())
            }
            TensorTerm::HigherRdm { min_order, product } => {
                write!(f, "HigherRDM(order={min_order}, fragment={product})")
            }
        }
    }
}

fn reduce_reference_term(
    term: &NormalOrderedTerm,
) -> Result<Vec<SignedTensorTerm>, TensorSimplifyError> {
    let mut common_factors = Vec::new();

    for delta in term.deltas() {
        if delta.is_contradictory() {
            return Ok(Vec::new());
        }
        common_factors.push(TensorTerm::Delta(
            delta.left().clone(),
            delta.right().clone(),
        ));
    }

    let residual_terms = reduce_normal_ordered_product(term.product())?;

    if residual_terms.is_empty() {
        if common_factors.is_empty() {
            return Ok(Vec::new());
        }

        let assembled = if common_factors.len() == 1 {
            common_factors.pop().unwrap()
        } else {
            TensorTerm::Product(common_factors)
        };

        return Ok(vec![SignedTensorTerm::new(term.coefficient(), assembled)]);
    }

    let mut reduced = Vec::with_capacity(residual_terms.len());
    for residual in residual_terms {
        let mut factors = common_factors.clone();
        factors.push(residual.term);

        let assembled = if factors.len() == 1 {
            factors.pop().unwrap()
        } else {
            TensorTerm::Product(factors)
        };

        reduced.push(SignedTensorTerm::new(
            term.coefficient() * residual.coefficient,
            assembled,
        ));
    }

    Ok(reduced)
}

fn reduce_normal_ordered_product(
    product: &OperatorProduct,
) -> Result<Vec<SignedTensorTerm>, TensorSimplifyError> {
    let ops = product.ops();

    if ops.is_empty() {
        return Ok(Vec::new());
    }

    if !is_normal_ordered(ops) {
        return Err(TensorSimplifyError::UnsupportedReferenceCase);
    }

    if ops
        .iter()
        .any(|op| matches!(op.index().space(), IndexSpace::Virtual))
    {
        return Ok(Vec::new());
    }

    match ops.len() {
        2 => reduce_one_body_product(ops),
        4 => reduce_two_body_product(product),
        len if len % 2 == 0 => reduce_higher_body_product(product, len / 2),
        _ => Err(TensorSimplifyError::UnsupportedOperatorRank),
    }
}

fn reduce_one_body_product(
    ops: &[crate::ast::FermionOp],
) -> Result<Vec<SignedTensorTerm>, TensorSimplifyError> {
    if ops[0].kind() != FermionOpKind::Create || ops[1].kind() != FermionOpKind::Annihilate {
        return Err(TensorSimplifyError::UnsupportedReferenceCase);
    }

    let left = ops[0].index().clone();
    let right = ops[1].index().clone();
    if matches!(left.space(), IndexSpace::General) || matches!(right.space(), IndexSpace::General) {
        return Err(TensorSimplifyError::UnsupportedReferenceCase);
    }

    match classify_one_body_spaces(left.space(), right.space()) {
        OneBodyReduction::Delta => Ok(vec![SignedTensorTerm::new(
            1,
            TensorTerm::Delta(left, right),
        )]),
        OneBodyReduction::Gamma => Ok(vec![SignedTensorTerm::new(
            1,
            TensorTerm::Gamma(left, right),
        )]),
        OneBodyReduction::Zero => Ok(Vec::new()),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OneBodyReduction {
    Delta,
    Gamma,
    Zero,
}

fn classify_one_body_spaces(left: IndexSpace, right: IndexSpace) -> OneBodyReduction {
    match (left, right) {
        (IndexSpace::Core, IndexSpace::Core) => OneBodyReduction::Delta,
        (IndexSpace::Active, IndexSpace::Active) => OneBodyReduction::Gamma,
        (IndexSpace::Virtual, IndexSpace::Virtual) => OneBodyReduction::Zero,
        _ => OneBodyReduction::Zero,
    }
}

fn reduce_two_body_product(
    product: &OperatorProduct,
) -> Result<Vec<SignedTensorTerm>, TensorSimplifyError> {
    let ops = product.ops();
    if ops[0].kind() != FermionOpKind::Create
        || ops[1].kind() != FermionOpKind::Create
        || ops[2].kind() != FermionOpKind::Annihilate
        || ops[3].kind() != FermionOpKind::Annihilate
    {
        return Err(TensorSimplifyError::UnsupportedReferenceCase);
    }

    if ops
        .iter()
        .any(|op| matches!(op.index().space(), IndexSpace::General))
    {
        return Err(TensorSimplifyError::UnsupportedReferenceCase);
    }

    let core_creator_positions = positions_for_space(ops, 0..2, IndexSpace::Core);
    let core_annihilator_positions = positions_for_space(ops, 2..4, IndexSpace::Core);
    let active_creator_positions = positions_for_space(ops, 0..2, IndexSpace::Active);
    let active_annihilator_positions = positions_for_space(ops, 2..4, IndexSpace::Active);

    if active_creator_positions.len() == 2 && active_annihilator_positions.len() == 2 {
        return Ok(vec![SignedTensorTerm::new(
            1,
            TensorTerm::Gamma2(
                ops[0].index().clone(),
                ops[1].index().clone(),
                ops[2].index().clone(),
                ops[3].index().clone(),
            ),
        )]);
    }

    if core_creator_positions.len() == 2 && core_annihilator_positions.len() == 2 {
        let first = TensorTerm::Product(vec![
            TensorTerm::Delta(ops[0].index().clone(), ops[3].index().clone()),
            TensorTerm::Delta(ops[1].index().clone(), ops[2].index().clone()),
        ]);
        let second = TensorTerm::Product(vec![
            TensorTerm::Delta(ops[0].index().clone(), ops[2].index().clone()),
            TensorTerm::Delta(ops[1].index().clone(), ops[3].index().clone()),
        ]);

        return Ok(vec![
            SignedTensorTerm::new(1, first),
            SignedTensorTerm::new(-1, second),
        ]);
    }

    if core_creator_positions.len() == 1
        && core_annihilator_positions.len() == 1
        && active_creator_positions.len() == 1
        && active_annihilator_positions.len() == 1
    {
        let grouped_positions = [
            core_creator_positions[0],
            core_annihilator_positions[0],
            active_creator_positions[0],
            active_annihilator_positions[0],
        ];
        let sign = permutation_sign(&grouped_positions);
        let product = TensorTerm::Product(vec![
            TensorTerm::Delta(
                ops[core_creator_positions[0]].index().clone(),
                ops[core_annihilator_positions[0]].index().clone(),
            ),
            TensorTerm::Gamma(
                ops[active_creator_positions[0]].index().clone(),
                ops[active_annihilator_positions[0]].index().clone(),
            ),
        ]);

        return Ok(vec![SignedTensorTerm::new(sign, product)]);
    }

    Ok(Vec::new())
}

fn reduce_higher_body_product(
    product: &OperatorProduct,
    order: usize,
) -> Result<Vec<SignedTensorTerm>, TensorSimplifyError> {
    let ops = product.ops();
    let is_active_higher_body = ops
        .iter()
        .take(order)
        .all(|op| op.kind() == FermionOpKind::Create)
        && ops
            .iter()
            .skip(order)
            .all(|op| op.kind() == FermionOpKind::Annihilate)
        && ops
            .iter()
            .all(|op| matches!(op.index().space(), IndexSpace::Active));

    if !is_active_higher_body {
        return Err(TensorSimplifyError::UnsupportedReferenceCase);
    }

    Ok(vec![SignedTensorTerm::new(
        1,
        TensorTerm::HigherRdm {
            min_order: order,
            product: product.clone(),
        },
    )])
}

fn is_normal_ordered(ops: &[crate::ast::FermionOp]) -> bool {
    ops.windows(2)
        .all(|pair| match (pair[0].kind(), pair[1].kind()) {
            (FermionOpKind::Create, FermionOpKind::Annihilate) => true,
            (FermionOpKind::Create, FermionOpKind::Create)
            | (FermionOpKind::Annihilate, FermionOpKind::Annihilate) => {
                pair[0].index().symbol() <= pair[1].index().symbol()
            }
            (FermionOpKind::Annihilate, FermionOpKind::Create) => false,
        })
}

fn collapse_signed_terms(
    terms: Vec<SignedTensorTerm>,
) -> Result<SimplifiedTensorForm, TensorSimplifyError> {
    let mut non_zero = terms
        .into_iter()
        .filter(|term| term.coefficient != 0)
        .collect::<Vec<_>>();

    match non_zero.len() {
        0 => Ok(SimplifiedTensorForm::new(TensorTerm::Zero)),
        1 => {
            let only = non_zero.pop().unwrap();
            if only.coefficient == 1 {
                Ok(SimplifiedTensorForm::new(only.term))
            } else {
                Ok(SimplifiedTensorForm::new(TensorTerm::Sum(vec![only])))
            }
        }
        _ => Ok(SimplifiedTensorForm::new(TensorTerm::Sum(non_zero))),
    }
}

fn positions_for_space(
    ops: &[crate::ast::FermionOp],
    range: std::ops::Range<usize>,
    space: IndexSpace,
) -> Vec<usize> {
    range
        .filter(|&idx| ops[idx].index().space() == space)
        .collect()
}

fn permutation_sign(order: &[usize]) -> i8 {
    let mut inversions = 0usize;
    for i in 0..order.len() {
        for j in (i + 1)..order.len() {
            if order[i] > order[j] {
                inversions += 1;
            }
        }
    }

    if inversions.is_multiple_of(2) { 1 } else { -1 }
}
