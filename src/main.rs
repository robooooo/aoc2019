pub mod days;
pub mod error;
pub mod utils;
pub mod intcode;

use days::nine as day;

fn main() -> utils::Result<()> {
    //stderrlog::new().verbosity(0).init()?;

    let first = day::first()?;
    println!("{}", first);
    let second = day::second()?;
    println!("{}", second);

    Ok(())
}   