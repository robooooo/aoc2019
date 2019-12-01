use std::{
    fs::File,
    io::{prelude::*, BufReader},
    str::FromStr,
    path::Path
};

/// Construct a non-relative path from a relative path
/// 
/// rel: relative path from project root
///
/// returns: non-relative path from /
pub fn path(rel: &str) -> String {
    format!("/home/april/Documents/Programming/aoc2019/src/inp/{}", rel)
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
