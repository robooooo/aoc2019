pub mod days;
pub mod utils;

use days::four;

fn main() -> utils::Result<()> {
    let (first, second) = four::solve()?;
    println!("{}", first);
    println!("{}", second);

    Ok(())
}