pub mod days;
pub mod utils;

use days::four;

fn main() -> utils::Result<()> {
    println!("{}", four::first()?);

    Ok(())
}
