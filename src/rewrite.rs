use std::cmp::Ordering;
use std::fmt;

use crate::ast::{FermionOp, FermionOpKind, Index, IndexSpace, OperatorProduct};
use crate::constraints::DeltaConstraint;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SimplifyConfig;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimplifyError {
    EmptyOperatorProduct,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalOrderedExpr {
    terms: Vec<NormalOrderedTerm>,
}

impl NormalOrderedExpr {
    pub fn new(terms: Vec<NormalOrderedTerm>) -> Self {
        Self { terms }
    }

    pub fn terms(&self) -> &[NormalOrderedTerm] {
        &self.terms
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalOrderedTerm {
    coefficient: i8,
    deltas: Vec<DeltaConstraint>,
    product: OperatorProduct,
}

impl NormalOrderedTerm {
    pub fn new(coefficient: i8, deltas: Vec<DeltaConstraint>, product: OperatorProduct) -> Self {
        Self {
            coefficient,
            deltas,
            product,
        }
    }

    pub fn coefficient(&self) -> i8 {
        self.coefficient
    }

    pub fn deltas(&self) -> &[DeltaConstraint] {
        &self.deltas
    }

    pub fn product(&self) -> &OperatorProduct {
        &self.product
    }
}

pub type SimplifiedOperatorForm = NormalOrderedExpr;
pub type SimplifiedTerm = NormalOrderedTerm;

pub fn simplify_operator_form(
    product: OperatorProduct,
    _config: SimplifyConfig,
) -> Result<NormalOrderedExpr, SimplifyError> {
    normal_order_product(product)
}

pub fn normal_order_product(product: OperatorProduct) -> Result<NormalOrderedExpr, SimplifyError> {
    if product.ops().is_empty() {
        return Err(SimplifyError::EmptyOperatorProduct);
    }

    let initial = NormalOrderedTerm::new(1, Vec::new(), product);
    let mut terms = normal_order_term(initial);
    combine_like_terms(&mut terms);

    Ok(NormalOrderedExpr::new(terms))
}

impl fmt::Display for NormalOrderedExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, term) in self.terms.iter().enumerate() {
            let coefficient = term.coefficient();
            let is_negative = coefficient < 0;
            let abs = coefficient.unsigned_abs();
            let body = render_term_body(term);

            if idx == 0 {
                if is_negative {
                    write!(f, "- ")?;
                }
            } else if is_negative {
                write!(f, " - ")?;
            } else {
                write!(f, " + ")?;
            }

            if abs != 1 || body == "1" {
                write!(f, "{abs}")?;
                if body != "1" {
                    write!(f, " ")?;
                }
            }

            if body != "1" || abs == 1 {
                write!(f, "{body}")?;
            }
        }

        Ok(())
    }
}

fn normal_order_term(term: NormalOrderedTerm) -> Vec<NormalOrderedTerm> {
    if term.coefficient() == 0 {
        return Vec::new();
    }

    match first_rewrite_action(term.product()) {
        None => vec![canonicalize_term(term)],
        Some(RewriteAction::Zero) => Vec::new(),
        Some(RewriteAction::SameKindSwap(position)) => {
            let swapped = swap_adjacent(term.product(), position);
            normal_order_term(NormalOrderedTerm::new(
                -term.coefficient(),
                term.deltas().to_vec(),
                swapped,
            ))
        }
        Some(RewriteAction::MixedSwap(position)) => {
            let ops = term.product().ops();
            let left = ops[position].index().clone();
            let right = ops[position + 1].index().clone();

            let mut delta_constraints = term.deltas().to_vec();
            delta_constraints.push(DeltaConstraint::new(left, right));

            let delta_branch = NormalOrderedTerm::new(
                term.coefficient(),
                delta_constraints,
                remove_adjacent_pair(term.product(), position),
            );
            let swapped_branch = NormalOrderedTerm::new(
                -term.coefficient(),
                term.deltas().to_vec(),
                swap_adjacent(term.product(), position),
            );

            let mut terms = normal_order_term(delta_branch);
            terms.extend(normal_order_term(swapped_branch));
            terms
        }
    }
}

fn canonicalize_term(mut term: NormalOrderedTerm) -> NormalOrderedTerm {
    let mut deltas = term.deltas;
    deltas.sort_by(compare_delta_constraints);
    term.deltas = deltas;
    term.product = term.product.canonicalize();
    term
}

fn combine_like_terms(terms: &mut Vec<NormalOrderedTerm>) {
    terms.sort_by(compare_terms);

    let mut combined = Vec::with_capacity(terms.len());
    for term in terms.drain(..) {
        if let Some(last) = combined.last_mut()
            && same_structure(last, &term)
        {
            last.coefficient += term.coefficient;
            continue;
        }
        combined.push(term);
    }

    combined.retain(|term| term.coefficient != 0);
    *terms = combined;
}

fn same_structure(left: &NormalOrderedTerm, right: &NormalOrderedTerm) -> bool {
    left.deltas == right.deltas && left.product == right.product
}

fn compare_terms(left: &NormalOrderedTerm, right: &NormalOrderedTerm) -> Ordering {
    left.product()
        .ops()
        .len()
        .cmp(&right.product().ops().len())
        .then_with(|| compare_delta_slices(left.deltas(), right.deltas()))
        .then_with(|| left.product().to_string().cmp(&right.product().to_string()))
}

fn compare_delta_slices(left: &[DeltaConstraint], right: &[DeltaConstraint]) -> Ordering {
    left.len().cmp(&right.len()).then_with(|| {
        left.iter()
            .map(render_delta)
            .collect::<Vec<_>>()
            .cmp(&right.iter().map(render_delta).collect::<Vec<_>>())
    })
}

fn compare_delta_constraints(left: &DeltaConstraint, right: &DeltaConstraint) -> Ordering {
    compare_index(left.left(), right.left())
        .then_with(|| compare_index(left.right(), right.right()))
}

fn compare_index(left: &Index, right: &Index) -> Ordering {
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

enum RewriteAction {
    Zero,
    SameKindSwap(usize),
    MixedSwap(usize),
}

fn first_rewrite_action(product: &OperatorProduct) -> Option<RewriteAction> {
    product
        .ops()
        .windows(2)
        .enumerate()
        .find_map(|(position, pair)| classify_adjacent_pair(position, &pair[0], &pair[1]))
}

fn classify_adjacent_pair(
    position: usize,
    left: &FermionOp,
    right: &FermionOp,
) -> Option<RewriteAction> {
    if left.kind() == right.kind() && left.index() == right.index() {
        return Some(RewriteAction::Zero);
    }

    match (left.kind(), right.kind()) {
        (FermionOpKind::Annihilate, FermionOpKind::Create) => {
            Some(RewriteAction::MixedSwap(position))
        }
        (kind, other_kind)
            if kind == other_kind && compare_index(left.index(), right.index()).is_gt() =>
        {
            Some(RewriteAction::SameKindSwap(position))
        }
        _ => None,
    }
}

fn swap_adjacent(product: &OperatorProduct, position: usize) -> OperatorProduct {
    let mut ops = product.ops().to_vec();
    ops.swap(position, position + 1);
    OperatorProduct::new(ops)
}

fn remove_adjacent_pair(product: &OperatorProduct, position: usize) -> OperatorProduct {
    let mut ops = product.ops().to_vec();
    ops.remove(position + 1);
    ops.remove(position);
    OperatorProduct::new(ops)
}

fn render_term_body(term: &NormalOrderedTerm) -> String {
    let mut factors = term.deltas().iter().map(render_delta).collect::<Vec<_>>();
    if !term.product().ops().is_empty() {
        factors.push(term.product().to_string());
    }

    if factors.is_empty() {
        "1".to_string()
    } else {
        factors.join(" ")
    }
}

fn render_delta(delta: &DeltaConstraint) -> String {
    format!(
        "delta({},{})",
        delta.left().symbol(),
        delta.right().symbol()
    )
}
