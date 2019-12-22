use crate::{utils, intcode::{Intcode, eval}};

pub fn first() -> utils::Result<i128> {
    let inp = utils::get_split(",", utils::path("nine.txt"))?;
    let mut cpu = Intcode::new(inp);
    for a in crate::intcode::interpreter::eval_all(&mut cpu, 1).unwrap() {
        print!("{} ", a);
    }
    Ok(0)
}   