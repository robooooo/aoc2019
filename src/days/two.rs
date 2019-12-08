use crate::{
    error::TwoError::*,
    utils,
};

pub fn compute(mem: &mut Vec<i32>, noun: i32, verb: i32) -> utils::Result<i32> {
    mem[1] = noun;
    mem[2] = verb;

    let mut pc = 0;
    loop {
        let op = mem[pc];
        if op == 99 {
            break;
        }
        let v_idx = mem[pc + 3] as usize;
        let x = mem[mem[pc + 1] as usize];
        let y = mem[mem[pc + 2] as usize];
        let v = &mut mem[v_idx];
        match op {
            1 => *v = x + y,
            2 => *v = x * y,
            x => return Err(InvalidOpcode(x))?,
        }
        pc += 4;
    }
    Ok(mem[0])
}

pub fn first() -> utils::Result<i32> {
    let mut mem: Vec<i32> = utils::get_split(",", utils::path("two.txt"))?;
    compute(&mut mem, 2, 12)
}

pub fn second() -> utils::Result<i32> {
    let mem: Vec<i32> = utils::get_split(",", utils::path("two.txt"))?;
    // Not useful, but a fun exercise in futility
    // (0..100)
    //     .zip(0..100)
    //     .map(|(n, v)| (n, v, compute(&mut mem.clone(), n, v)))
    //     .filter_map(|(n, v, x)| match x {
    //         Ok(x) if x == 19690720 => Some(100 * n + v),
    //         _ => None,
    //     })
    //     .next()
    //     .ok_or(Box::new(NoSolutions))
    for noun in 0..100 {
        for verb in 0..100 {
            let ret = compute(&mut mem.clone(), noun, verb)?;
            if ret == 19690720 {
                return Ok(100 * noun + verb);
            }
        }
    }
    Err(Box::new(NoSolutions))
}
