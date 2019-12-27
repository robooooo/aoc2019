use crate::{utils, intcode::{Intcode, Int, self}};
use std::iter;

pub fn first() -> utils::Result<Int> {
    let inp = utils::get_split(",", utils::path("nine.txt"))?;
    let mut cpu = Intcode::new(inp);
    for a in intcode::eval(&mut cpu, iter::once(1)).unwrap() {
        print!("{} ", a);
    }
    Ok(0)
}   