use crate::{intcode::Intcode, utils};

pub fn first() -> utils::Result<String> {

    let input = utils::get_split(",", utils::path("five.txt"))?;
    let mut res = String::new();
    let mut cpu = Intcode::new(input, 1);

    while cpu.running() {
        cpu.step();
        if let Some(out) = cpu.out() {
            res.push_str(&format!("{} ", out));
        }
    }

    Ok(res)
}

pub fn second() -> utils::Result<String> {

    let input = utils::get_split(",", utils::path("five.txt"))?;
    let mut res = String::new();
    let mut cpu = Intcode::new(input, 5);

    while cpu.running() {
        cpu.step();
        if let Some(out) = cpu.out() {
            res.push_str(&format!("{} ", out));
        }
    }

    Ok(res)
}
