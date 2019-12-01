use crate::utils;

pub fn first() -> Result<i64, Box<dyn std::error::Error>> {
    let nums: Vec<i64> = utils::get_lines(utils::path("one.txt"))?;
    let res = nums.iter().map(|x| (x / 3) - 2).fold(0, |acc, x| acc + x);
    Ok(res)
}

pub fn second() -> Result<i64, Box<dyn std::error::Error>> {
    let mut nums: Vec<i64> = utils::get_lines(utils::path("one.txt"))?;
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
