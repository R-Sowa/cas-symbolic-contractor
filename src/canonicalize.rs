use crate::ast::{FermionOp, FermionOpKind, OperatorProduct};

fn sort_key(op: &FermionOp) -> (u8, &str) {
    let kind_rank = match op.kind() {
        FermionOpKind::Create => 0,
        FermionOpKind::Annihilate => 1,
    };

    (kind_rank, op.index().symbol())
}

pub fn canonicalize_operator_product(product: &OperatorProduct) -> OperatorProduct {
    let mut ops = product.ops().to_vec();
    ops.sort_by(|lhs, rhs| sort_key(lhs).cmp(&sort_key(rhs)));
    OperatorProduct::new(ops)
}
