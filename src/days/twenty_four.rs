use crate::{error::TwentyFourError, utils};
use std::{collections::HashSet, fmt, fmt::Display, str::FromStr};

const ADJ_TILES: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Copy, Clone, Eq, Debug, PartialEq, Hash)]
enum Tile {
    Infested,
    Clean,
}

impl FromStr for Tile {
    type Err = TwentyFourError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "#" => Tile::Infested,
            "." => Tile::Clean,
            _ => return Err(TwentyFourError::ParseError),
        };
        Ok(res)
    }
}

struct Tiles(Vec<Tile>);

impl FromStr for Tiles {
    type Err = TwentyFourError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = Vec::new();
        for chr in s.chars() {
            res.push(chr.to_string().parse()?);
        }
        Ok(Tiles(res))
    }
}

#[derive(Clone, Eq, Debug, PartialEq, Hash)]
struct Board {
    data: Vec<Vec<Tile>>,
}

impl Display for Board {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut res = String::new();
        for row in &self.data {
            for val in row {
                res.push(match val {
                    Tile::Infested => '#',
                    Tile::Clean => '.',
                });
            }
            res.push('\n');
        }
        res.pop();

        write!(fmt, "{}", res)
    }
}

impl Board {
    fn new(data: Vec<Vec<Tile>>) -> Self {
        Board { data }
    }

    fn get(&self, x: usize, y: usize) -> Option<Tile> {
        self.data.get(y).and_then(|y| y.get(x)).cloned()
    }

    fn get_i32(&self, x: i32, y: i32) -> Option<Tile> {
        if x < 0 || y < 0 {
            None
        } else {
            self.get(x as usize, y as usize)
        }
    }

    fn biodiversity(&self) -> u32 {
        self.data.iter().flatten().enumerate().fold(0, |acc, (n, x)| {
            match x {
                Tile::Infested => acc + 2u32.pow(n as u32),
                Tile::Clean => acc,
            }
        })
    }

    fn step(&mut self) {
        let mut new: Vec<Vec<Tile>> = self.data.clone();

        for (y, row) in new.iter_mut().enumerate() {
            for (x, val) in row.iter_mut().enumerate() {
                let sum: i32 = ADJ_TILES
                    .iter()
                    .map(|(dx, dy)| {
                        let x = x as i32 + dx;
                        let y = y as i32 + dy;
                        match self.get_i32(x, y) {
                            Some(Tile::Infested) => 1,
                            _ => 0,
                        }
                    })
                    .sum();
                
                *val = match val {
                    Tile::Infested if sum != 1 => Tile::Clean,
                    Tile::Clean if (sum == 1 || sum == 2) => Tile::Infested,
                    _  => *val,
                }
            }
        }

        self.data = new;
    }
}

pub fn first() -> utils::Result<u32> {
    let input: Vec<Tiles> = utils::get_lines(utils::path("twenty_four.txt"))?;
    let input: Vec<Vec<Tile>> = input.into_iter().map(|x| x.0).collect();
    let mut board = Board::new(input);
    let mut set = HashSet::new();

    loop {
        if set.contains(&board) {
            return Ok(board.biodiversity());
        }
        set.insert(board.clone());
        board.step();
    }
}
