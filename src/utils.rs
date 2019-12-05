use std::{
    error::Error,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    str::FromStr,
    num::ParseIntError,
};
use err_derive::Error;

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

/// Type alias for all results in aoc2019
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Construct a non-relative path from a relative path
///
/// rel: relative path from project root
///
/// returns: non-relative path from /
pub fn path(rel: &str) -> String {
    format!("/home/april/Documents/Programming/aoc2019/src/inp/{}", rel)
}

/// Attempt to interpret file as split-seperated several lines of T.
///
/// path: Filepath to read contents of.
///
/// returns: Vec<T> is the result of converting each line in path to T
pub fn get_split<T, I>(
    split: &str,
    path: I,
) -> Result<Vec<T>>
where
    T: FromStr,
    I: AsRef<Path>,
    <T as std::str::FromStr>::Err: Error + 'static,
{
    let mut vec = vec![];
    let mut f = File::open(path)?;
    let mut text = String::new();
    f.read_to_string(&mut text)?;

    for val in text.split(split) {
        vec.push(val.parse()?);
    }

    Ok(vec)
}

/// Attempt to interpret file as several lines of T.
///
/// path: Filepath to read contents of.
///
/// returns: Vec<T> is the result of converting each line in path to T
pub fn get_lines<T, I>(path: I) -> Result<Vec<T>>
where
    T: FromStr,
    I: AsRef<Path>,
    <T as std::str::FromStr>::Err: std::error::Error + 'static,
{
    let mut vec = vec![];
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        vec.push(line?.parse()?);
    }

    Ok(vec)
}

/// From bottom digit up to top
/// Where digits[0] is the lower digit
pub fn digits(mut num: i32) -> Vec<i32> {
    let mut digits = Vec::new();
    while num > 0 {
        digits.insert(0, num % 10);
        num -= num % 10;
        num /= 10;
    }
    digits
}