use super::var::Var;
use std::{fmt, ops};

pub trait Expr: Clone + PartialEq + Eq + fmt::Debug {}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ExprAdd<Lhs: Expr, Rhs: Expr> {
    pub(crate) lhs: Lhs,
    pub(crate) rhs: Rhs,
}

impl<X: Expr, Y: Expr> Expr for ExprAdd<X, Y> {}

impl<EL: Expr, ER: Expr> ops::Add<Var> for ExprAdd<EL, ER> {
    type Output = ExprAdd<Self, Var>;

    fn add(self, rhs: Var) -> Self::Output {
        ExprAdd { lhs: self, rhs }
    }
}

impl<EL: Expr, ER: Expr> ops::Add<ExprAdd<EL, ER>> for Var {
    type Output = ExprAdd<Self, ExprAdd<EL, ER>>;

    fn add(self, rhs: ExprAdd<EL, ER>) -> Self::Output {
        ExprAdd { lhs: self, rhs }
    }
}

impl<Lhs: Expr, Rhs: Expr> fmt::Debug for ExprAdd<Lhs, Rhs> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} + {:?}", self.lhs, self.rhs)
    }
}

#[test]
fn binop() {
    let x = Var::new();
    let y = Var::new();
    let u = Var::new();
    let z = x + y;
    let a = z + y;
    assert_eq!(z, x + y);
    assert_ne!(z, x + u);
    assert_eq!(a, z + y);
}
