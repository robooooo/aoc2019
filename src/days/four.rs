use crate::utils;
use itertools::Itertools;

const MIN: i32 = 138307;
const MAX: i32 = 654504;

/// From bottom digit up to top
/// Where digits[0] is the lower digit
fn digits(mut num: i32) -> Vec<i32> {
    let mut digits = Vec::new();
    while num > 0 {
        digits.insert(0, num % 10);
        num -= num % 10;
        num /= 10;
    }
    digits
}

pub fn solve() -> utils::Result<(i32, i32)> {
    let mut sol_one = 0;
    let mut sol_two = 0;
    for n in MIN..=MAX {
        let digits = digits(n);

        let increasing = digits.windows(2).all(|w| w[0] <= w[1]);
        let equal = digits.windows(2).any(|w| w[0] == w[1]);
        let grouped = correct_group(&digits);

        if increasing && equal {
            sol_one += 1;
        }
        if increasing && grouped {
            sol_two += 1;
        }
    }
    Ok( (sol_one, sol_two) )
}

pub fn correct_group(digits: &[i32]) -> bool {
    for (_, group) in &digits.into_iter().group_by(|x| *x) {
        if group.count() == 2 {
            return true;
        }
    }

    false    
}