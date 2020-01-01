use err_derive::Error;
use crate::intcode::{structs::Instruction, Int};

#[derive(Copy, Clone, Debug, Error, Eq, PartialEq)]
pub enum IntcodeErr {
    #[error(display = "Interpreter tried to write to an argument in direct addressing mode")]
    WriteDirect,
    #[error(display = "Attempted to access a negative address")]
    NegativeAccess,
    #[error(display = "Unknown instruction")]
    UnknownInstruction(Int),
    #[error(display = "Unknown addressing mode")]
    UnknownMode,
    #[error(display = "An eval- function ran out of arguments")]
    EvalNoArgs,
}

