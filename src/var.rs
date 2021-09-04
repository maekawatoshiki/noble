use super::{
    expr::Expr,
    ty::{Type, I32},
};
use std::{
    cell::RefCell,
    clone::Clone,
    cmp::{Eq, PartialEq},
    fmt,
    marker::PhantomData,
};

thread_local! {
    pub static ID: RefCell<u64> = RefCell::new(1);
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Var<T: Type> {
    id: u64,
    _marker: PhantomData<fn() -> T>,
}

impl Var<I32> {
    #[cfg(test)]
    pub fn new() -> Self {
        Var {
            id: ID.with(|f| {
                let id = *f.borrow();
                *f.borrow_mut() += 1;
                id
            }),
            _marker: PhantomData,
        }
    }
}

impl<T: Type> Expr<T> for Var<T> {}

impl<T: Type> fmt::Debug for Var<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Var({})", self.id)
    }
}
