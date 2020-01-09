use crate::{
    display::PixBuf,
    error::FifteenError,
    inp,
    intcode::{Int, Intcode},
    out_or, utils,
};
use crossterm::{
    event::{self, Event, KeyCode},
    style::Color,
};
use num::{FromPrimitive};
use num_derive::{FromPrimitive, ToPrimitive};
use std::io::stdout;

const SIZE: i32 = 64;

pub fn first() -> utils::Result<i32> {
    let input = utils::get_split(",", utils::path("fifteen.txt"))?;
    let mut display = PixBuf::<std::io::Stdout>::new(stdout())?;
    let mut cpu = Intcode::new(input);
    let (mut x, mut y) = (0, 0);

    display.fill(0, 0, SIZE as u16, SIZE as u16, Color::Black)?;

    loop {
        let mov = loop {
            let event = match event::read()? {
                Event::Key(e) => e,
                _ => continue,
            };

            break match event.code {
                KeyCode::Up => Direction::South,
                KeyCode::Down => Direction::North,
                KeyCode::Left => Direction::West,
                KeyCode::Right => Direction::East,
                KeyCode::Char('q') => Err(FifteenError::Terminated)?,
                _ => continue,
            };
        };
        let (dx, dy) = mov.into();

        inp!(cpu, mov as Int);
        let status = out_or!(cpu);
        let status = Status::from_i128(status).ok_or(FifteenError::InvalidStatus(status))?;

        match status {
            Status::Wall => display.show((SIZE / 2 + x + dx) as u16, (SIZE / 2 + y + dy) as u16, Color::White)?,
            Status::Moved => {
                display.show((SIZE / 2 + x) as u16, (SIZE / 2 + y) as u16, Color::Black)?;
                display.show((SIZE / 2 + x + dx) as u16, (SIZE / 2 + y + dy) as u16, Color::Red)?;
                x += dx;
                y += dy;
            }
            Status::MovedOxygen => {
                display.show((SIZE / 2 + x) as u16, (SIZE / 2 + y) as u16, Color::Black)?;
                display.show((SIZE / 2 + x + dx) as u16, (SIZE / 2 + y + dy) as u16, Color::Blue)?;
            }
        }
    }

    Ok(0)
}

#[derive(Copy, Clone, FromPrimitive, ToPrimitive)]
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

#[derive(Copy, Clone, FromPrimitive, ToPrimitive)]
enum Status {
    Wall = 0,
    Moved = 1,
    MovedOxygen = 2,
}
