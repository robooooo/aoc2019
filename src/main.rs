pub mod days;
pub mod utils;

use days::three;

fn main() -> utils::Result<()> {
    let (a, b) = three::solve()?;
    println!("{}", a);
    println!("{}", b);

    Ok(())
}
