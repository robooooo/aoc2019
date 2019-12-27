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
    UnexpectedDigit,
}

#[derive(Error, Debug)]
pub enum TwentyTwoFromStrErr {
    #[error(display = "Invalid technique")]
    InvalidTechnique,
    #[error(display = "Range error")]
    RangeError,
    #[error(display = "Parse error: {:?}", _0)]
    ParseError(ParseIntError),
}

impl From<ParseIntError> for TwentyTwoFromStrErr {
    fn from(e: ParseIntError) -> Self {
        TwentyTwoFromStrErr::ParseError(e)
    }
}

#[derive(Error, Debug)]
pub enum TwentyTwoError {
    #[error(display = "No solutions")]
    NoSolutions,
    #[error(display = "Parse error: {:?}", _0)]
    ParseError(TwentyTwoFromStrErr),
}

#[derive(Error, Debug)]
pub enum TwentyFourError {
    #[error(display = "Pattern did not match a cell")]
    ParseError,
}