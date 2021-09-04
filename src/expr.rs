use super::var::Var;
use std::{clone::Clone, cmp::Eq, fmt, marker::PhantomData, ops};

pub trait Expr<T>: Clone + PartialEq + Eq + fmt::Debug
where
    T: Clone + PartialEq + Eq + fmt::Debug,
{
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ExprAdd<T, Lhs, Rhs>
where
    T: Clone + PartialEq + Eq + fmt::Debug,
    Lhs: Expr<T>,
    Rhs: Expr<T>,
{
    pub(crate) lhs: Lhs,
    pub(crate) rhs: Rhs,
    pub(crate) _marker: PhantomData<fn() -> T>,
}

impl<T, X, Y> Expr<T> for ExprAdd<T, X, Y>
where
    T: Clone + PartialEq + Eq + fmt::Debug,
    X: Expr<T>,
    Y: Expr<T>,
{
}

impl<T, EL, ER> ops::Add<Var> for ExprAdd<T, EL, ER>
where
    T: Clone + PartialEq + Eq + fmt::Debug,
    EL: Expr<T>,
    ER: Expr<T>,
{
    type Output = ExprAdd<T, Self, Var>;

    fn add(self, rhs: Var) -> Self::Output {
        ExprAdd {
            lhs: self,
            rhs,
            _marker: PhantomData,
        }
    }
}

impl<T, EL, ER> ops::Add<ExprAdd<T, EL, ER>> for Var
where
    T: Clone + PartialEq + Eq + fmt::Debug,
    EL: Expr<T>,
    ER: Expr<T>,
{
    type Output = ExprAdd<T, Self, ExprAdd<T, EL, ER>>;

    fn add(self, rhs: ExprAdd<T, EL, ER>) -> Self::Output {
        ExprAdd {
            lhs: self,
            rhs,
            _marker: PhantomData,
        }
    }
}

impl<T, Lhs, Rhs> fmt::Debug for ExprAdd<T, Lhs, Rhs>
where
    T: Clone + PartialEq + Eq + fmt::Debug,
    Lhs: Expr<T>,
    Rhs: Expr<T>,
{
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
