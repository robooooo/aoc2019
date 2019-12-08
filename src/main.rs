pub mod days;
pub mod error;
pub mod utils;
pub mod intcode;

use days::eight;

fn main() -> utils::Result<()> {
    //stderrlog::new().verbosity(0).init()?;

    let first = eight::first()?;
    println!("{}", first);
    let second = eight::second()?;
    println!("{}", second);

    Ok(())
}   