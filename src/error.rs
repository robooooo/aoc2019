use err_derive::Error;
use std::num::ParseIntError;

#[derive(Error, Debug)]
pub enum TwoError {
    #[error(display = "Invalid opcode (got {:?})", _0)]
    InvalidOpcode(i32),
    #[error(display = "No solutions found")]
    NoSolutions,
}

#[derive(Error, Debug)]
pub enum ThreeError {
    #[error(display = "Invalid direction (got {:?})", _0)]
    InvalidDirection(char),
    #[error(display = "Got an empty instruction while parsing")]
    EmptyDirection,
    #[error(display = "Error parsing int: {}", _0)]
    ParseError(ParseIntError),
    #[error(display = "No solutions found")]
    NoSolutions,
}

impl From<ParseIntError> for ThreeError {
    fn from(error: ParseIntError) -> Self {
        ThreeError::ParseError(error)
    }
}

#[derive(Error, Debug)]
pub enum EightError {
    #[error(display = "No solutions found")]
    NoSolutions,
    #[error(display = "Unexpected digit encountered in input")]
    UnexpectedDigit
}