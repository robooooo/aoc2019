use crate::{error::EightError, utils};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

pub fn first() -> utils::Result<i32> {
    let inp: Vec<i32> = utils::get_char_nums(utils::path("eight.txt"))?;

    let mut count = std::i32::MAX;
    let mut sol = 0;
    for layer in inp.chunks(WIDTH * HEIGHT) {
        let zeroes = count_digits(0, layer);
        if zeroes < count {
            count = zeroes;
            sol = count_digits(1, layer) * count_digits(2, layer);
        }
    }

    Ok(sol)
}

pub fn second() -> utils::Result<String> {
    let inp: Vec<i32> = utils::get_char_nums(utils::path("eight.txt"))?;

    let mut img = vec![vec![' '; WIDTH]; HEIGHT];
    for layer in inp.chunks(WIDTH * HEIGHT) {
        for (idx, val) in layer.iter().enumerate() {
            if img[idx / WIDTH][idx % WIDTH] == ' ' {
                match val {
                    0 => img[idx / WIDTH][idx % WIDTH] = '░',
                    1 => img[idx / WIDTH][idx % WIDTH] = '█',
                    2 => continue,
                    _ => Err(EightError::UnexpectedDigit)?,
                }
            }
        }
    }

    let res = img
        .into_iter()
        .map(|line| line.into_iter().collect::<String>())
        .fold(String::new(), |acc, line| format!("{}\n{}", acc, line));
    Ok(res)
}

pub fn count_digits(digit: i32, iter: &[i32]) -> i32 {
    let mut count = 0;
    for item in iter {
        if *item == digit {
            count += 1;
        }
    }
    count
}
