pub mod interpreter;
// pub mod iterator;
pub mod error;
pub mod eval;
pub mod input;
mod structs;

pub use self::{
    eval::{eval, eval_all, eval_args, eval_once},
    interpreter::{Int, Intcode},
};
