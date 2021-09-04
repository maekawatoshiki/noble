use super::{expr::Expr, var::Var};
use std::fmt;

pub struct Function<Args> {
    args: Args,
    body: Expr,
}

impl Function<(Var, Var)> {
    pub fn new(args: (Var, Var), body: Expr) -> Self {
        Self { args, body }
    }

    pub fn realize(&self) {}
}

impl fmt::Debug for Function<(Var, Var)> {
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

#[test]
fn realize() {
    use crate::buffer::Buffer;

    let x = Var::new();
    let y = Var::new();
    let c = Var::new();
    let buf = Buffer::new();
    let val = buf.at(x, y, c);
    let val = val * 2;
    let f = Function::new((x, y), val);
    dbg!(f);
}
