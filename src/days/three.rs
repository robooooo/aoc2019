use crate::utils::{
    self,
    ThreeError::{self, *},
};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Movement {
    direction: Direction,
    distance: i32,
}

impl FromStr for Movement {         
    type Err = ThreeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;

        // Parse direction by looking at first char
        let mut chars = s.chars();
        let dir = if let Some(chr) = chars.next() {
            match chr {
                'R' => Right,
                'L' => Left,
                'D' => Up,
                'U' => Down,
                _ => return Err(InvalidDirection(chr)),
            }
        } else {
            return Err(EmptyDirection);
        };

        let dis: i32 = chars.collect::<String>().parse()?;

        let res = Movement {
            direction: dir,
            distance: dis,
        };
        Ok(res)
    }
}

// returns (first answer, second answer)
pub fn solve() -> utils::Result<(i32, i32)> {
    let input: Vec<String> = utils::get_lines(utils::path("three.txt"))?;
    let first: Vec<Movement> = input[0]
        .split(",")
        .filter_map(|mov: &str| mov.parse().ok())
        .collect();
    let second: Vec<Movement> = input[1]
        .split(",")
        .filter_map(|mov: &str| mov.parse().ok())
        .collect();

    let mut fst_set: HashSet<(i32, i32)> = HashSet::new();
    let mut snd_set: HashSet<(i32, i32)> = HashSet::new();
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    for (wire, set) in &mut [(first, &mut fst_set), (second, &mut snd_set)] {
        let mut steps = 0;
        let mut x = 0;
        let mut y = 0;
        for mov in wire {
            while mov.distance != 0 {
                steps += 1;
                match mov.direction {
                    Direction::Up => y += 1,
                    Direction::Down => y -= 1,
                    Direction::Left => x -= 1,
                    Direction::Right => x += 1,
                }
                let new = (x, y);
                set.insert(new);
                map.entry(new).and_modify(|v| *v += steps).or_insert(steps);
                mov.distance -= 1;
            }
        }
    }

    let sols_one = fst_set.intersection(&snd_set);
    let sols_two = sols_one.clone();

    let sol_one = sols_one
        .into_iter()
        .map(|p| p.0.abs() + p.1.abs())
        .min()
        .ok_or(ThreeError::NoSolutions)?;

    let sol_two = sols_two
        .into_iter()
        .map(|p| map[p])
        .min()
        .ok_or(ThreeError::NoSolutions)?;

    Ok((sol_one, sol_two))
}
