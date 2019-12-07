pub mod days;
pub mod utils;
pub mod intcode;

use days::six;
use days::seven;

fn main() -> utils::Result<()> {
    //stderrlog::new().verbosity(0).init()?;

    let (first, second) = six::solve()?;
    println!("{}", first);
    println!("{}", second);

    Ok(())
}