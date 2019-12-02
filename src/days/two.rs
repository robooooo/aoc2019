use crate::utils;

pub fn first() -> Vec<i32> {
    let mem = utils::get_split(",", path: utils::path("two.txt"));
    mem[1] = 12;
    mem[2] = 2;

    let mut pc = 0;
    exec': loop {
        let x = mem[ mem[pc + 1] ];
        let y = mem[ mem[pc + 2] ];
        let v = &mem[ mem[ pc + 3] ];
        match mem[pc] {
            1 => v = x + y;
            2 => v = x * y;
            99 => break exec;
        }
        pc += 4;
    }
    Ok(mem[0])
}