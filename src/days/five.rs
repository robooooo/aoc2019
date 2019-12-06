use crate::{intcode::Intcode, utils};

pub fn first() -> utils::Result<String> {

    let input = utils::get_split(",", utils::path("five.txt"))?;
    let mut res = String::new();
    let mut cpu = Intcode::new(input);

    while cpu.running() {
        cpu.step();
        if let Some(out) = cpu.out() {
            res.push_str(&format!("{} ", out));
        }
    }

    Ok(res)
}
