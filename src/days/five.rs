use crate::{intcode::Intcode, utils};

fn one() -> utils::Result<String> {

    let input = utils::get_split(",", utils::path("five.txt"))?;
    let res = String::new();
    let cpu = Intcode::new(input);


    Ok(res)
}
