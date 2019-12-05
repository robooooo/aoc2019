pub mod days;
pub mod utils;
pub mod intcode;

use days::four;
use days::five;

fn main() -> utils::Result<()> {
    let (first, second) = four::solve()?;
    println!("{}", first);
    println!("{}", second);

    Ok(())
}