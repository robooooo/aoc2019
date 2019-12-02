pub mod days;
pub mod utils;

use days::one;
use days::two;

fn main() -> utils::Result<()> {
    println!("{}", two::first());

    Ok(())
}
