use crate::{
    intcode::{self, Int, Intcode, interpreter},
    utils,
};
use std::iter;

pub fn first() -> utils::Result<String> {
    let inp = utils::get_split(",", utils::path("nine.txt"))?;
    let mut cpu = Intcode::new(inp);
    let mut out = String::new();
    for i in intcode::eval(&mut cpu, iter::once(1))? {
        out.push_str(&i.to_string());
    }
    Ok(out)
}

pub fn second() -> utils::Result<Int> {
    let inp = utils::get_split(",", utils::path("nine.txt"))?;
    let mut cpu = Intcode::new(inp);
    Ok(interpreter::eval_args(&mut cpu, iter::once(2))?)
}
