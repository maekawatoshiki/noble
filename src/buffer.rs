use crate::{gen_id, var::Var};

#[derive(Clone, Copy, Debug)]
pub struct Buffer {
    pub id: u64,
}

#[derive(Clone, Copy, Debug)]
pub struct PointedBuffer {
    pub buf: Buffer,
    pub x: Var,
    pub y: Var,
    pub c: Var,
}

impl Buffer {
    pub fn new() -> Self {
        Self { id: gen_id() }
    }

    pub fn at(self, x: Var, y: Var, c: Var) -> PointedBuffer {
        PointedBuffer::new(self, x, y, c)
    }
}

impl PointedBuffer {
    pub fn new(buf: Buffer, x: Var, y: Var, c: Var) -> Self {
        Self { buf, x, y, c }
    }
}
