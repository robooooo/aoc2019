pub mod days;
pub mod utils;
pub mod intcode;

use days::seven;

fn main() -> utils::Result<()> {
    //stderrlog::new().verbosity(0).init()?;

    let first = seven::first()?;
    println!("{}", first);
    let second = seven::second()?;
    println!("{}", second);

    Ok(())
}