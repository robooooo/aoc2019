mod days;
pub mod utils;

use days::one;

fn main() -> Result<(), Box::<dyn std::error::Error>> {
    println!("{}", one::first()?);
    println!("{}", one::second()?);
    println!("{}", one::big()?);

    Ok(())
}
