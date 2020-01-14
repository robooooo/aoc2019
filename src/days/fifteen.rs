use crate::{
    inp,
    intcode::{error::IntcodeErr, Int, Intcode},
    out, utils,
};
use num::FromPrimitive;
use num_derive::{FromPrimitive, ToPrimitive};
use std::collections::{HashSet, VecDeque};

pub fn first() -> utils::Result<usize> {
    let input = utils::get_split(",", utils::path("fifteen.txt"))?;
    let mut queue = VecDeque::new();
    queue.push_back(VecDeque::new());
    let cpu = Intcode::new(input);
    let mut set = HashSet::new();

    while let Some(attempt) = queue.pop_front() {
        // dbg!(&attempt);
        match try_input(&mut cpu.clone(), &attempt)? {
            Status::Moved => {
                use Direction::*;
                for &dir in &[North, East, South, West] {
                    let mut at = attempt.clone();
                    at.push_back(dir);
                    if !set.contains(&at) {
                        queue.push_back(at);
                    } else {
                        set.insert(at);
                    }
                }
            }
            Status::MovedOxygen => return Ok(attempt.len()),
            Status::Wall => continue,
        }
    }

    Ok(0)
}

fn try_input(cpu: &mut Intcode, instrs: &VecDeque<Direction>) -> utils::Result<Status> {
    let mut last = Some(Status::Moved as Int);
    for &mov in instrs.iter() {
        inp!(cpu, mov as Int);
        last = out!(cpu);
    }
    last.and_then(Status::from_i128)
        .ok_or(IntcodeErr::EvalNoArgs)
        .map_err(Into::into)
}

#[derive(Copy, Clone, Eq, Debug, Hash, FromPrimitive, ToPrimitive, PartialEq)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl From<Direction> for (i32, i32) {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::North => (0, 1),
            Direction::South => (0, -1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
enum Status {
    Wall = 0,
    Moved = 1,
    MovedOxygen = 2,
}
