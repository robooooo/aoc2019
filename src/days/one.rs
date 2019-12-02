use crate::utils;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};
use num_bigint::BigUint;
use num_traits::Zero;

pub fn first() -> Result<i64, Box<dyn std::error::Error>> {
    let nums: Vec<i64> = utils::get_lines(utils::path("one.txt"))?;
    let res = nums.iter().fold(0, |acc, x| acc + (x / 3 + 2));
    Ok(res)
}

pub fn second() -> Result<i64, Box<dyn std::error::Error>> {
    let nums: Vec<i64> = utils::get_lines(utils::path("one.txt"))?;
    let res = nums.iter().fold(0, |acc, x| {
        let mut x = *x;
        let mut sum = 0;
        while x > 0 {
            x = (x / 3) - 2;
            sum += if x > 0 { x } else { 0 };
        }
        acc + sum
    });
    Ok(res)
}

pub fn big() -> Result<BigUint, Box<dyn std::error::Error>> {
    let f = File::open(utils::path("one_big.txt"))?;
    let reader = BufReader::new(f);

    let two: BigUint = BigUint::from(2u8);
    let three: BigUint = BigUint::from(3u8);

    let mut res = Zero::zero(); 
    for line in reader.lines() {
        res += line?.parse::<BigUint>()? / &three - &two;
    }
    Ok(res)
}
