use crate::{
    intcode::{Intcode, interpreter::State},
    utils,
};

pub fn first() -> utils::Result<String> {
    let input = utils::get_split(",", utils::path("five.txt"))?;
    let mut res = String::new();
    let mut cpu = Intcode::new(input);

    loop {
        match cpu.step() {
            State::Running => continue,
            State::Output(out) => res.push_str(&format!("{} ", out)),
            State::Waiting => cpu.input(1),
            _ => break,
        }
    }

    Ok(res)
}

pub fn second() -> utils::Result<String> {
    let input = utils::get_split(",", utils::path("five.txt"))?;
    let mut res = String::new();
    let mut cpu = Intcode::new(input);

    loop {
        match cpu.step() {
            State::Running => continue,
            State::Output(out) => res.push_str(&format!("{} ", out)),
            State::Waiting => cpu.input(5),
            _ => break,
        }
    }

    Ok(res)
}
