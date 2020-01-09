pub mod days;
pub mod error;
pub mod utils;
pub mod intcode;
pub mod display;

<<<<<<< HEAD
use days::fifteen as day;
=======
use days::ten as day;
>>>>>>> 1e95e33c3d93095ced17c886a75dacdc981c6dd3

fn main() -> utils::Result<()> {
    //stderrlog::new().verbosity(0).init()?;

    let (first, second) = day::first()?;
    println!("{}", first);
<<<<<<< HEAD
    // let second = day::second()?;
=======
    // let second = eight::second()?;
>>>>>>> 1e95e33c3d93095ced17c886a75dacdc981c6dd3
    // println!("{}", second);

    Ok(())
}   