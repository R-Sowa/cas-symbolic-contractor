use crate::ast::{Index, IndexSpace};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeltaConstraint {
    left: Index,
    right: Index,
}

impl DeltaConstraint {
    pub fn new(left: Index, right: Index) -> Self {
        Self { left, right }
    }

    pub fn left(&self) -> &Index {
        &self.left
    }

    pub fn right(&self) -> &Index {
        &self.right
    }

    pub fn is_contradictory(&self) -> bool {
        match (self.left.space(), self.right.space()) {
            (IndexSpace::General, _) | (_, IndexSpace::General) => false,
            (left, right) => left != right,
        }
    }
}
