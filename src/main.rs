pub mod days;
pub mod error;
pub mod utils;
pub mod intcode;
pub mod display;

use days::ten as day;

fn main() -> utils::Result<()> {
    //stderrlog::new().verbosity(0).init()?;

    let (first, second) = day::first()?;
    println!("{}", first);
    // let second = eight::second()?;
    // println!("{}", second);

    Ok(())
}   