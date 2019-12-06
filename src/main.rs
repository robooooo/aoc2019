pub mod days;
pub mod utils;
pub mod intcode;

use days::five;

fn main() -> utils::Result<()> {
    let first = five::first()?;
    println!("{}", first);

    Ok(())
}