use super::{
    expr::{Expr, ExprAdd},
    ty::Type,
};
use std::{
    cell::RefCell,
    clone::Clone,
    cmp::{Eq, PartialEq},
    fmt,
    marker::PhantomData,
    ops,
};

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

impl<T: Type> Expr<T> for Var {}

impl ops::Add for Var {
    type Output = ExprAdd<(), Var, Var>;

    fn add(self, rhs: Self) -> Self::Output {
        ExprAdd {
            lhs: self,
            rhs,
            _marker: PhantomData,
        }
    }
}

impl fmt::Debug for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Var({})", self.id)
    }
}
