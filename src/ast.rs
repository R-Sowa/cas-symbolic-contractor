use std::fmt;
use std::ops::Mul;

use crate::canonicalize::canonicalize_operator_product;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexSpace {
    Core,
    Active,
    Virtual,
    General,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Index {
    symbol: String,
    space: IndexSpace,
}

impl Index {
    pub fn new(symbol: impl Into<String>, space: IndexSpace) -> Self {
        Self {
            symbol: symbol.into(),
            space,
        }
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn space(&self) -> IndexSpace {
        self.space
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FermionOpKind {
    Create,
    Annihilate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FermionOp {
    kind: FermionOpKind,
    index: Index,
}

impl FermionOp {
    pub fn new(kind: FermionOpKind, index: Index) -> Self {
        Self { kind, index }
    }

    pub fn kind(&self) -> FermionOpKind {
        self.kind
    }

    pub fn index(&self) -> &Index {
        &self.index
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OperatorProduct {
    ops: Vec<FermionOp>,
}

impl OperatorProduct {
    pub fn new(ops: Vec<FermionOp>) -> Self {
        Self { ops }
    }

    pub fn ops(&self) -> &[FermionOp] {
        &self.ops
    }

    pub fn canonicalize(&self) -> Self {
        canonicalize_operator_product(self)
    }
}

impl Mul<FermionOp> for FermionOp {
    type Output = OperatorProduct;

    fn mul(self, rhs: FermionOp) -> Self::Output {
        OperatorProduct::new(vec![self, rhs])
    }
}

impl Mul<FermionOp> for OperatorProduct {
    type Output = OperatorProduct;

    fn mul(mut self, rhs: FermionOp) -> Self::Output {
        self.ops.push(rhs);
        self
    }
}

impl fmt::Display for OperatorProduct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, op) in self.ops.iter().enumerate() {
            if idx > 0 {
                write!(f, " ")?;
            }

            match op.kind() {
                FermionOpKind::Create => write!(f, "a†({})", op.index().symbol())?,
                FermionOpKind::Annihilate => write!(f, "a({})", op.index().symbol())?,
            }
        }

        Ok(())
    }
}
