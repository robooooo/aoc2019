use err_derive::Error;
use std::num::ParseIntError;
use crate::intcode::error::IntcodeErr;

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
pub enum ElevenError {
    #[error(display = "Output ({}) was not a valid tile", _0)]
    InvalidTile(i128),
    #[error(display = "Output ({}) was not a valid turn", _0)]
    InvalidTurn(i128),
    #[error(display = "Error with brain: {}", _0)]
    InterpreterError(IntcodeErr),
}

// impl From<i128> for ElevenError {
//     fn from(e: i128) -> Self {
//         ElevenError::InvalidTile(e)
//     }
// }

#[derive(Error, Debug)]
pub enum ThirteenError {
    #[error(display = "Output ({}) was not a valid tile", _0)]
    InvalidTile(i128),
}

#[derive(Error, Debug)]
pub enum FifteenError {
    #[error(display = "Output ({}) was not a valid direction", _0)]
    InvalidDirection(i128),
    #[error(display = "Output ({}) was not a valid status", _0)]
    InvalidStatus(i128),
    #[error(display = "Process was manually quit")]
    Terminated,
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