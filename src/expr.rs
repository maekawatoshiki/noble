use super::{ty::Type, var::Var};
use std::{clone::Clone, cmp::Eq, fmt, marker::PhantomData, ops};

pub trait Expr<T>: Clone + fmt::Debug
where
    T: Type,
{
}

#[derive(Copy, Clone)]
pub struct ExprBinOp<T, Lhs, Rhs>
where
    T: Type,
    Lhs: Expr<T>,
    Rhs: Expr<T>,
{
    pub(crate) kind: BinOpKind,
    pub(crate) lhs: Lhs,
    pub(crate) rhs: Rhs,
    pub(crate) _marker: PhantomData<fn() -> T>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BinOpKind {
    Add,
    Mul,
}

#[derive(Copy, Clone)]
pub struct ExprCast<FROM, TO, E>
where
    FROM: Type,
    TO: Type,
    E: Expr<FROM>,
{
    pub(crate) expr: E,
    pub(crate) _marker: PhantomData<fn() -> FROM>,
    pub(crate) _marker2: PhantomData<fn() -> TO>,
}

impl<FROM, TO, E> ExprCast<FROM, TO, E>
where
    FROM: Type,
    TO: Type,
    E: Expr<FROM>,
{
    pub fn new(expr: E) -> Self {
        Self {
            expr,
            _marker: PhantomData,
            _marker2: PhantomData,
        }
    }
}

impl<T, X, Y> Expr<T> for ExprBinOp<T, X, Y>
where
    T: Type,
    X: Expr<T>,
    Y: Expr<T>,
{
}

macro_rules! operator {
    ($n1:ident, $n2:ident) => {
        impl<E1, E2, T> ops::$n1<Var<T>> for ExprBinOp<T, E1, E2>
        where
            T: Type,
            E1: Expr<T>,
            E2: Expr<T>,
        {
            type Output = ExprBinOp<T, Self, Var<T>>;

            fn $n2(self, rhs: Var<T>) -> Self::Output {
                ExprBinOp {
                    kind: BinOpKind::$n1,
                    lhs: self,
                    rhs,
                    _marker: PhantomData,
                }
            }
        }

        impl<E1, E2, E3, E4, T> ops::$n1<ExprBinOp<T, E1, E2>> for ExprBinOp<T, E3, E4>
        where
            T: Type,
            E1: Expr<T>,
            E2: Expr<T>,
            E3: Expr<T>,
            E4: Expr<T>,
        {
            type Output = ExprBinOp<T, Self, ExprBinOp<T, E1, E2>>;

            fn $n2(self, rhs: ExprBinOp<T, E1, E2>) -> Self::Output {
                ExprBinOp {
                    kind: BinOpKind::$n1,
                    lhs: self,
                    rhs,
                    _marker: PhantomData,
                }
            }
        }

        impl<T> ops::$n1<Var<T>> for Var<T>
        where
            T: Type,
        {
            type Output = ExprBinOp<T, Self, Var<T>>;

            fn $n2(self, rhs: Var<T>) -> Self::Output {
                ExprBinOp {
                    kind: BinOpKind::$n1,
                    lhs: self,
                    rhs,
                    _marker: PhantomData,
                }
            }
        }

        impl<T, E1, E2> ops::$n1<ExprBinOp<T, E1, E2>> for Var<T>
        where
            T: Type,
            E1: Expr<T>,
            E2: Expr<T>,
        {
            type Output = ExprBinOp<T, Self, ExprBinOp<T, E1, E2>>;

            fn $n2(self, rhs: ExprBinOp<T, E1, E2>) -> Self::Output {
                ExprBinOp {
                    kind: BinOpKind::$n1,
                    lhs: self,
                    rhs,
                    _marker: PhantomData,
                }
            }
        }
    };
}

operator!(Add, add);
operator!(Mul, mul);

impl<T, Lhs, Rhs> fmt::Debug for ExprBinOp<T, Lhs, Rhs>
where
    T: Type,
    Lhs: Expr<T>,
    Rhs: Expr<T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} {} {:?}",
            self.lhs,
            match self.kind {
                BinOpKind::Add => "+",
                BinOpKind::Mul => "*",
            },
            self.rhs
        )
    }
}

#[test]
fn binop() {
    let x = Var::new();
    let y = Var::new();
    let _u = Var::new();
    let z = x + y;
    let a = z + y;
    let b = z + a;
    let _c = b * a + x;
}

#[test]
fn cast() {
    use super::ty::F64;
    let x = Var::new();
    let y = ExprCast::<_, F64, _>::new(x);
    // let _z = y + x;
}
