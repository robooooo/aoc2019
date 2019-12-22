use crate::{
    error::{TwentyTwoError, TwentyTwoFromStrErr},
    utils,
};
use lazy_static::lazy_static;
use regex::Regex;
use std::{ops::Range, str::FromStr};

lazy_static! {
    static ref DEAL_NEW: Regex = Regex::new(r"^deal into new stack$").unwrap();
    static ref DEAL_WITH: Regex = Regex::new(r"^deal with increment ([\+\-0-9]{1,})$").unwrap();
    static ref CUT: Regex = Regex::new(r"^cut ([\+\-0-9]{1,})$").unwrap();
}

enum Technique {
    DealNew,
    CutTop(usize),
    CutBottom(usize),
    DealWith(usize),
}

impl FromStr for Technique {
    type Err = TwentyTwoFromStrErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Technique::*;
        if DEAL_NEW.is_match(s) {
            Ok(DealNew)
        } else if let Some(cap) = DEAL_WITH.captures(s) {
            let n_str = cap.get(1).unwrap();
            let n = FromStr::from_str(n_str.as_str())?;
            Ok(DealWith(n))
        } else if let Some(cap) = CUT.captures(s) {
            let n_str = cap.get(1).unwrap();
            let n: i64 = FromStr::from_str(n_str.as_str())?;
            if n > 0 {
                Ok(CutTop(n as usize))
            } else {
                Ok(CutBottom(-n as usize))
            }
        } else {
            Err(TwentyTwoFromStrErr::InvalidTechnique)
        }
    }
}

pub fn first() -> utils::Result<usize> {
    let input: Vec<Technique> = utils::get_lines(utils::path("twenty_two.txt"))?;
    let mut cards: Vec<i32> = (0..10007).collect();
    for line in input {
        match line {
            Technique::DealNew => cards.reverse(),
            Technique::CutTop(n) => cards.rotate_left(n),
            Technique::CutBottom(n) => cards.rotate_right(n),
            Technique::DealWith(n) => {
                let mut new = vec![0; cards.len()];
                let mut idx = 0;
                for card in &cards {
                    new[idx] = *card;
                    idx = (idx + n) % cards.len();
                }
                cards = new;
            }
        }
    }
    //Result: 9 2 5 8 1 4 7 0 3 6
    cards
        .iter()
        .position(|c| *c == 2019)
        .ok_or(TwentyTwoError::NoSolutions.into())
}

pub fn second() -> utils::Result<usize> {
    let input: Vec<Technique> = utils::get_lines(utils::path("twenty_two.txt"))?;
    let len = 101741582076661 - 1;
    let mut idx = 2020;
    let r: Range<usize> = 0..101741582076661;
    for _ in r {
        match line {
            Technique::DealNew => idx = len - idx,
            Technique::CutBottom(n) => idx = (idx + n) % len,
        }
    }

    //Result: 9 2 5 8 1 4 7 0 3 6
    Ok(idx)
}
