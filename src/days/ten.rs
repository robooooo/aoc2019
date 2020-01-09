use crate::utils;
use std::{collections::HashSet};

pub fn first() -> utils::Result<(i32, i32)> {
    let lines: Vec<String> = utils::get_lines(utils::path("ten.txt"))?;
    let mut asteroids = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, chr) in line.chars().enumerate() {
            if chr == '#' {
                asteroids.push((x as i32, y as i32));
            }
        }
    }
    let (sx, sy) = get_station(&asteroids); 

    asteroids.sort_unstable_by(|x, y| {
        let theta_x = (2.0f64).atan();

        let dx = x.0.abs() + x.1.abs();
        let dy = y.0.abs() + y.1.abs();
        dx.cmp(&dy)
    });

    let mut sol_one = None;
    let mut sol_two = 0;
    let mut set = HashSet::new();
    let mut to_remove = Vec::new();
    while !asteroids.is_empty() {
        for (x, y) in &asteroids {
            let mut diff = Difference::new(sx - x, sy - y);
            diff.simplify();
            if set.contains(&diff) {
                to_remove.push((*x, *y));
            } else {
                set.insert(diff);
            }
        }
        sol_one.get_or_insert(set.len());

        for coord in to_remove.drain(..) {
            asteroids = asteroids.into_iter().filter(|x| *x != coord).collect();
        }
    }


    unimplemented!()
}

fn get_station(asteroids: &[(i32, i32)]) -> (i32, i32) {
    let mut max = 0;
    let mut station = (0, 0);
    for (x0, y0) in asteroids.iter() {
        let mut set = HashSet::new();
        for (x1, y1) in asteroids.iter() {
            if x0 == x1 && y0 == y1 {
                continue;
            }
            let mut diff = Difference::new(x0 - x1, y0 - y1);
            diff.simplify();
            set.insert(diff);
        }
        if set.len() > max {
            max = set.len();
            station = (*x0, *y0);
        }
    }
    station
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Difference {
    dx: i32,
    dy: i32,
}

impl Difference {
    fn new(dx: i32, dy: i32) -> Self {
        Difference { dx, dy }
    }

    fn simplify(&mut self) {
        let gcd = gcd(self.dx, self.dy);
        self.dx /= gcd;
        self.dy /= gcd;
    }
}

fn gcd(x: i32, y: i32) -> i32 {
    let mut min = x.min(y);
    let mut max = x.max(y);

    while min != 0 {
        let r = max % min;
        max = min;
        min = r;
    }

    max.abs()
}
