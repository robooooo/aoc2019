use crate::intcode::{self, Intcode, interpreter::State};
use crate::utils;
use itertools::Itertools;

pub fn first() -> utils::Result<i32> {
    let inp = utils::get_split(",", utils::path("seven.txt"))?;

    let sol = (0..=4)
        .permutations(5)
        .filter_map(|seq| {
            let mut last = 0;
            for i in 0..5 {
                let mut curr = Intcode::new(inp.clone());
                last = match intcode::eval_args(&mut curr, &vec![seq[i], last]) {
                    Some(res) => res,
                    None => return None,
                }
            }
            Some(last)
        })
        .max()
        .unwrap_or(0);

    Ok(sol)
}

pub fn second() -> utils::Result<i32> {
    let inp = utils::get_split(",", utils::path("seven.txt"))?;

    let sol = (5..=9)
        .permutations(5)
        .filter_map(|seq| {
            let mut amps = Vec::with_capacity(5);
            for i in 0..5 {
                amps.push(Intcode::new(inp.clone()));
                amps[i].input(seq[i]);
            }

            let mut acc = 0;
            'halt: for i in (0..5).cycle() {
                acc = loop {
                    match amps[i].step() {
                        State::Running => continue,
                        State::Output(out) => break out,
                        State::Waiting => amps[i].input(acc),
                        State::Halted => break 'halt,
                        _ => return None,
                    }
                }
            }

            Some(acc)
        })
        .max()
        .unwrap_or(0);

    Ok(sol)
}