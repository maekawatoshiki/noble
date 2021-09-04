use std::{cell::RefCell, fmt, ops};

thread_local! {
    pub static ID: RefCell<u64> = RefCell::new(1);
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Var {
    id: u64,
}

impl Var {
    #[cfg(test)]
    fn new() -> Self {
        Var {
            id: ID.with(|f| {
                let id = *f.borrow();
                *f.borrow_mut() += 1;
                id
            }),
        }
    }
}

trait Expr: Clone + PartialEq + Eq + fmt::Debug {}

#[derive(Copy, Clone, PartialEq, Eq)]
struct ExprAdd<LHS: Expr, RHS: Expr> {
    lhs: LHS,
    rhs: RHS,
}

impl Expr for Var {}
impl<X: Expr, Y: Expr> Expr for ExprAdd<X, Y> {}

impl ops::Add for Var {
    type Output = ExprAdd<Var, Var>;

    fn add(self, rhs: Self) -> Self::Output {
        ExprAdd { lhs: self, rhs }
    }
}

impl ops::Add<Var> for ExprAdd<Var, Var> {
    type Output = ExprAdd<Self, Var>;

    fn add(self, rhs: Var) -> Self::Output {
        ExprAdd { lhs: self, rhs }
    }
}

impl ops::Add<ExprAdd<Var, Var>> for Var {
    type Output = ExprAdd<Self, ExprAdd<Var, Var>>;

    fn add(self, rhs: ExprAdd<Var, Var>) -> Self::Output {
        ExprAdd { lhs: self, rhs }
    }
}

impl fmt::Debug for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Var({})", self.id)
    }
}

impl<LHS: Expr, RHS: Expr> fmt::Debug for ExprAdd<LHS, RHS> {
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
