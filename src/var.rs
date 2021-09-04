use super::expr::{Expr, ExprAdd};
use std::{cell::RefCell, fmt, ops};

thread_local! {
    pub static ID: RefCell<u64> = RefCell::new(1);
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Var {
    id: u64,
}

impl Var {
    #[cfg(test)]
    pub fn new() -> Self {
        Var {
            id: ID.with(|f| {
                let id = *f.borrow();
                *f.borrow_mut() += 1;
                id
            }),
        }
    }
}

impl Expr for Var {}

impl ops::Add for Var {
    type Output = ExprAdd<Var, Var>;

    fn add(self, rhs: Self) -> Self::Output {
        ExprAdd { lhs: self, rhs }
    }
}

impl fmt::Debug for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Var({})", self.id)
    }
}
