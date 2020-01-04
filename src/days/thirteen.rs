use crate::{
    display::PixBuf,
    error::ThirteenError,
    intcode::{error::IntcodeErr, interpreter::State, Int, Intcode},
    utils,
};
use crossterm::style::Color;
use num::FromPrimitive;
use num_derive::FromPrimitive;
use std::{
    thread,
    time::Duration,
    cmp::Ordering,
    collections::HashSet,
    io::{stdout, Stdout},
};

#[derive(Copy, Clone, Eq, FromPrimitive, PartialEq)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}

impl Tile {
    fn color(&self) -> Color {
        match *self {
            Tile::Empty => Color::Black,
            Tile::Wall => Color::White,
            Tile::Block => Color::Grey,
            Tile::Paddle => Color::DarkBlue,
            Tile::Ball => Color::Blue,
        }
    }
}

fn next_three(ic: &mut Intcode) -> Option<Result<(Int, Int, Int), IntcodeErr>> {
    let mut out = [0; 3];
    let mut idx = 0;

    loop {
        match ic.step() {
            State::Running => continue,
            State::Output(o) => {
                out[idx] = o;
                idx += 1;
                if idx >= 3 {
                    break;
                }
            }
            State::Waiting => {
                return Some(Err(IntcodeErr::EvalNoArgs));
            }
            State::Halted => return None,
            State::Error(e) => {
                return Some(Err(e));
            }
        }
    }

    let res = (out[0], out[1], out[2]);
    Some(Ok(res))
}

struct ArcadeGame {
    cpu: Intcode,
    err: Option<IntcodeErr>,
}

impl ArcadeGame {
    fn new(code: Vec<Int>) -> Self {
        ArcadeGame {
            cpu: Intcode::new(code),
            err: None,
        }
    }
}

impl Iterator for &mut ArcadeGame {
    type Item = (Int, Int, Int);
    fn next(&mut self) -> Option<Self::Item> {
        match next_three(&mut self.cpu) {
            Some(Ok(tuple)) => Some(tuple),
            Some(Err(e)) => {
                self.err = Some(e);
                None
            }
            None => None,
        }
    }
}

pub fn first() -> utils::Result<usize> {
    let input = utils::get_split(",", utils::path("thirteen.txt"))?;
    let mut game = ArcadeGame::new(input);
    let mut set = HashSet::new();

    for (x, y, id) in &mut game {
        if id == 2 {
            set.insert((x, y));
        }
    }

    if let Some(e) = game.err {
        Err(e)?;
    }

    Ok(set.len())
}

pub fn second() -> utils::Result<Int> {
    let mut input = utils::get_split(",", utils::path("thirteen.txt"))?;
    input[0] = 2;

    let mut res = 0;
    let mut ball_x = 0;
    let mut paddle_x = 0;
    let mut cpu = Intcode::new(input);
    let mut screen = PixBuf::<Stdout>::new(stdout())?;

    for ball_idx in 0.. {
        let (x, y, id) = match next_three(&mut cpu) {
            Some(res) => res?,
            None => break,
        };

        let tile = if x == -1 && y == 0 {
            res = id;
            continue;
        } else {
            let tile = Tile::from_i128(id).ok_or(ThirteenError::InvalidTile(id))?;
            screen.show(x as u16, y as u16, tile.color())?;
            tile
        };

        if tile == Tile::Paddle {
            paddle_x = x;
        } else if tile == Tile::Ball {
            ball_x = x;
        }

        let code = match ball_x.cmp(&paddle_x) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };

        if ball_idx > 995 {
            thread::sleep(Duration::from_millis(5));
        }

        cpu.input(code);
    }

    Ok(res)
}
