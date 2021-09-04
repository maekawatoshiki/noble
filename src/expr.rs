use super::{buffer::PointedBuffer, ty::Type, var::Var};
use std::{clone::Clone, ops};

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Cast(Box<Expr>),
    Var(Var),
    PointedBuffer(PointedBuffer),
    Int32(i32),
}

impl Expr {
    pub fn var(v: Var) -> Self {
        Self {
            ty: v.ty,
            kind: ExprKind::Var(v),
        }
    }

    pub fn pointed_buf(p: PointedBuffer) -> Self {
        Self {
            ty: Type::u8,
            kind: ExprKind::PointedBuffer(p),
        }
    }

    pub fn i32(i: i32) -> Self {
        Self {
            ty: Type::i32,
            kind: ExprKind::Int32(i),
        }
    }

    pub fn cast<E: Into<Expr>>(e: E, to: Type) -> Self {
        Self {
            ty: to,
            kind: ExprKind::Cast(Box::new(e.into())),
        }
    }
}

impl From<Var> for Expr {
    fn from(v: Var) -> Expr {
        Expr::var(v)
    }
}

impl From<PointedBuffer> for Expr {
    fn from(v: PointedBuffer) -> Expr {
        Expr::pointed_buf(v)
    }
}

impl<X: Into<Expr>> ops::Add<X> for Expr {
    type Output = Expr;

    fn add(self, rhs: X) -> Self::Output {
        let rhs = rhs.into();
        Expr {
            ty: rhs.ty,
            kind: ExprKind::Add(Box::new(self), Box::new(rhs)),
        }
    }
}

impl<X: Into<Expr>> ops::Add<X> for Var {
    type Output = Expr;

    fn add(self, rhs: X) -> Self::Output {
        let rhs = rhs.into();
        Expr {
            ty: self.ty,
            kind: ExprKind::Add(Box::new(Expr::var(self)), Box::new(rhs)),
        }
    }
}

impl ops::Mul<i32> for Expr {
    type Output = Expr;

    fn mul(self, rhs: i32) -> Self::Output {
        Expr {
            ty: self.ty,
            kind: ExprKind::Add(Box::new(self), Box::new(Expr::i32(rhs))),
        }
    }
}

impl ops::Mul<i32> for Var {
    type Output = Expr;

    fn mul(self, rhs: i32) -> Self::Output {
        Expr {
            ty: self.ty,
            kind: ExprKind::Add(Box::new(Expr::var(self)), Box::new(Expr::i32(rhs))),
        }
    }
}

impl ops::Mul<i32> for PointedBuffer {
    type Output = Expr;

    fn mul(self, rhs: i32) -> Self::Output {
        Expr {
            ty: Type::u8,
            kind: ExprKind::Add(Box::new(Expr::pointed_buf(self)), Box::new(Expr::i32(rhs))),
        }
    }
}

#[test]
fn binop() {
    let x = Var::new();
    let y = Var::new();
    let z = x + y;
    let u = z.clone() + x;
    let _h = u + z;
    // let _c = b * a + x;
    let _d = x * 2;
}

#[test]
fn cast() {
    let x = Var::new();
    let _y = Expr::cast(x, Type::f64);
    // let _z = y + x;
}
