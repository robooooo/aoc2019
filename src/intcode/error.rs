use err_derive::Error;

#[derive(Copy, Clone, Debug, Error, Eq, PartialEq)]
pub enum IntcodeErr {
    #[error(display = "Interpreter tried to write to an argument in direct addressing mode")]
    WriteDirect,
    #[error(display = "Out of bounds read")]
    ReadOob,
    #[error(display = "Out of bounds write")]
    WriteOob,
}