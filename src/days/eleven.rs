use crate::{
    error::ElevenError,
    intcode::{interpreter::State, Int, Intcode},
    utils,
};
use num::FromPrimitive;
use num_derive::{FromPrimitive, ToPrimitive};
use std::{collections::HashMap, convert::TryFrom};

#[derive(Copy, Clone, Eq, FromPrimitive, ToPrimitive, PartialEq)]
enum Tile {
    Black,
    White,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Black
    }
}

#[derive(FromPrimitive, ToPrimitive)]
enum Turn {
    Left,
    Right,
}

impl TryFrom<i128> for Turn {
    type Error = i128;
    fn try_from(x: i128) -> Result<Self, Self::Error> {
        Ok(match x {
            0 => Turn::Left,
            1 => Turn::Right,
            e => return Err(e),
        })
    }
}

#[derive(Copy, Clone, FromPrimitive, ToPrimitive)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(&mut self, turn: Turn) {
        let num = match turn {
            Turn::Left => 3,
            Turn::Right => 1,
        };
        *self = Direction::from_i8((*self as i8 + num) % 4).unwrap();
    }
}

pub fn first() -> utils::Result<usize> {
    let input = utils::get_split(",", utils::path("eleven.txt"))?;
    let tiles = solve(input, Tile::Black)?;
    Ok(tiles.len())
}

pub fn second() -> utils::Result<String> {
    let input = utils::get_split(",", utils::path("eleven.txt"))?;
    let tiles = solve(input, Tile::White)?;

    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = 0;
    let mut min_y = 0;

    for &(x, y) in tiles.keys() {
        max_x = max_x.max(x);
        max_y = max_y.max(y);
        min_x = min_x.min(x);
        min_y = min_y.min(y);
    }

    let dx = max_x - min_x + 1;
    let dy = min_x - min_y + 1;

    let mut pic = vec![vec![Tile::Black; dx as usize]; dy as usize];
    for ((x, y), tile) in tiles {
        if tile == Tile::White {
            let idx_x = (max_x - x) as usize;
            let idx_y = (max_y - y) as usize;
            pic[idx_y][idx_x] = tile;
        }
    }

    let mut res = String::new();
    pic.iter().for_each(|line| {
        line.iter().rev().for_each(|item| {
            res.push(match item {
                Tile::White => '█',
                Tile::Black => '░',
            });
        });
        res.push('\n');
    });
    res.pop();

    Ok(res)
}

fn solve(prog: Vec<Int>, init: Tile) -> Result<HashMap<(i32, i32), Tile>, ElevenError> {
    let mut brain = Intcode::new(prog);
    let mut tiles = HashMap::new();

    let mut dir = Direction::Up;
    let mut turn = false;

    let mut x = 0;
    let mut y = 0;

    tiles.insert((0, 0), init);

    loop {
        match brain.step() {
            State::Running => continue,
            State::Waiting => {
                let tile = *tiles.entry((x, y)).or_default();
                brain.input(tile as i128);
            }
            State::Output(out) => {
                if !turn {
                    let pos = (x, y);
                    *tiles.entry(pos).or_default() =
                        Tile::from_i128(out).ok_or(ElevenError::InvalidTile(out))?;
                } else {
                    let turn = Turn::from_i128(out).ok_or(ElevenError::InvalidTurn(out))?;
                    dir.turn(turn);
                    match dir {
                        Direction::Up => y += 1,
                        Direction::Right => x += 1,
                        Direction::Down => y -= 1,
                        Direction::Left => x -= 1,
                    }
                }
                turn = !turn;
            }
            State::Error(e) => return Err(ElevenError::InterpreterError(e)),
            State::Halted => break,
        }
    }

    Ok(tiles)
}
