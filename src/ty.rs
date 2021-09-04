use std::{clone::Clone, cmp::Eq, fmt};

pub trait Type: Copy + Clone + PartialEq + Eq + fmt::Debug {}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct F64;
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct U8;
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct I32;

impl Type for () {}
impl Type for U8 {}
impl Type for I32 {}
impl Type for F64 {}
