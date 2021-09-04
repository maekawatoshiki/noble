use super::{expr::Expr, var::Var};
use std::fmt;

pub struct Function<Args, E: Expr> {
    args: Args,
    body: E,
}

impl<E: Expr> Function<(Var, Var), E> {
    #[cfg(test)]
    fn new(args: (Var, Var), body: E) -> Self {
        Self { args, body }
    }
}

impl<E: Expr> fmt::Debug for Function<(Var, Var), E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Func({:?}, {:?}) = {:?}",
            self.args.0, self.args.1, self.body
        )
    }
}

#[test]
fn func() {
    let x = Var::new();
    let y = Var::new();
    let z = x + y;
    let f = Function::new((x, y), z);
    dbg!(f);
}
