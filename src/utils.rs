use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    str::FromStr,
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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
) -> std::result::Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: FromStr,
    I: AsRef<Path>,
    <T as std::str::FromStr>::Err: std::error::Error + 'static,
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
pub fn get_lines<T, I>(path: I) -> std::result::Result<Vec<T>, Box<dyn std::error::Error>>
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
