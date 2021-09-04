use super::{expr::Expr, ty::Type, var::Var};
use std::{fmt, marker::PhantomData};

pub struct Function<Args, T, E>
where
    T: Type,
    E: Expr<T>,
{
    args: Args,
    body: E,
    _marker: PhantomData<fn() -> T>,
}

impl<T, E> Function<(Var<T>, Var<T>), T, E>
where
    T: Type,
    E: Expr<T>,
{
    #[cfg(test)]
    fn new(args: (Var<T>, Var<T>), body: E) -> Self {
        Self {
            args,
            body,
            _marker: PhantomData,
        }
    }
}

impl<T, E> fmt::Debug for Function<(Var<T>, Var<T>), T, E>
where
    T: Type,
    E: Expr<T>,
{
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
