use crate::utils::{self, digits};

// macro_rules! arg {
//     ( $i:expr ) => {
//         )
//     };
// }

pub struct Intcode {
    mem: Vec<i32>,
    ip: usize,
}

impl Intcode {
    fn new(mem: Vec<i32>) -> Self {
        Intcode { mem: mem, ip: 0 }
    }

    fn address<'a>(&'a mut self, mode: AddrMode, value: &'a mut i32) -> &'a mut i32 {
        match mode {
            AddrMode::Position => &mut self.mem[*value as usize],
            AddrMode::Direct => value,
        }
    }

    fn step(&mut self) {}

    fn execute<'a>(&'a mut self, instr: Instruction) {
        
        match instr.op {
            Opcode::Add => 
        }
    }
}

struct Instruction {
    addr_modes: Vec<AddrMode>,
    num_args: i32,
    op: Opcode,
}

impl Into<Instruction> for i32 {
    fn into(self) -> Instruction {
        let op_num = self % 100;
        let op = Opcode::maybe_new(op_num).expect("Known opcode");
        let rem = (self / 100) - op_num;
        let digits = digits(rem);

        let num_args = match op {
            Opcode::Add => 3,
            Opcode::Mul => 3,
            Opcode::Inp => 1,
            Opcode::Out => 1,
            Opcode::Hlt => 0,
        };

        let mut addr_modes = Vec::new();
        for i in 0..num_args {
            addr_modes
                .push(AddrMode::maybe_new(*digits.get(i).unwrap_or(&0)).expect("Valid opcode"));
        }

        Instruction {
            addr_modes: addr_modes,
            num_args: num_args as i32,
            op: op,
        }
    }
}

enum Opcode {
    Add = 1,
    Mul = 2,
    Inp = 3,
    Out = 4,
    Hlt = 99,
}

impl Opcode {
    // Should be a better way to do this
    fn maybe_new(n: i32) -> Option<Opcode> {
        use Opcode::*;

        match n {
            1 => Some(Add),
            2 => Some(Mul),
            3 => Some(Inp),
            4 => Some(Out),
            99 => Some(Hlt),
            _ => None,
        }
    }
}

enum AddrMode {
    Position = 0,
    Direct = 1,
}

impl AddrMode {
    fn maybe_new(n: i32) -> Option<AddrMode> {
        use AddrMode::*;

        match n {
            0 => Some(Position),
            1 => Some(Direct),
            _ => None,
        }
    }
}
