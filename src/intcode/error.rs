use err_derive::Error;

#[derive(Copy, Clone, Debug, Error, Eq, PartialEq)]
pub enum IntcodeErr {
    #[error(display = "interpreter tried to write to an argument in direct addressing mode")]
    WriteDirect,
}