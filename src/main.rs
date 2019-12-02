pub mod days;
pub mod utils;

use days::two;

fn main() -> utils::Result<()> {
    println!("{}", two::first()?);
    println!("{}", two::second()?);

    Ok(())
}
