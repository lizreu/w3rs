#![feature(const_option)]

mod base;
mod gen;
mod macros;

pub mod backtrace;
pub mod consts;
pub mod stringret;

pub use base::*;
pub use gen::{handles::*, natives as native};
