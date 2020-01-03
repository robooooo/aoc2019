pub mod days;
pub mod error;
pub mod utils;
pub mod intcode;

use days::eleven as day;

fn main() -> utils::Result<()> {
    let first = day::first()?;
    println!("{}", first);
    let second = day::second()?;
    println!("{}", second);

    Ok(())
}   