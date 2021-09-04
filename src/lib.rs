#![feature(unboxed_closures, fn_traits)]

pub mod buffer;
pub mod expr;
pub mod function;
pub mod ty;
pub mod var;

use std::cell::RefCell;

thread_local! {
    pub static ID: RefCell<u64> = RefCell::new(1);
}

pub fn gen_id() -> u64 {
    ID.with(|f| {
        let id = *f.borrow();
        *f.borrow_mut() += 1;
        id
    })
}
