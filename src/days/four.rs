use crate::utils;

const MIN: i32 = 138307;
const MAX: i32 = 654504;

/// From bottom digit up to otp
/// Where digits[0] is the lower digit
pub fn digits(mut num: i32) -> Vec<i32> {
    let mut digits = Vec::new();
    while num > 0 {
        digits.insert(0, num % 10);
        num -= num % 10;
        num /= 10;
    }
    digits
}

pub fn first() -> utils::Result<i32> {
    println!("{:?}", digits(MIN));
    println!("{:?}", digits(MAX));


    let mut sol = 0;
    'main: for n in (MIN + 1)..MAX {
        let digits = digits(n);
        // Assert equality
        let increasing = digits.windows(2).all(|w| w[0] <= w[1]);
        let equal = digits.windows(2).any(|w| w[0] == w[1]);
        if increasing && equal {
            sol += 1;
        }
    }
    Ok(sol)
}