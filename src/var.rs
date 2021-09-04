use crate::{gen_id, ty::Type};
use std::{
    clone::Clone,
    cmp::{Eq, PartialEq},
    fmt,
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Var {
    pub id: u64,
    pub ty: Type,
}

impl Var {
    pub fn new() -> Self {
        Var {
            id: gen_id(),
            ty: Type::i32,
        }
    }
}

impl fmt::Debug for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Var({})", self.id)
    }
}
