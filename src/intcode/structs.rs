use crate::utils::digits;

pub(super) struct Instruction {
    pub(super) addr_modes: Vec<AddrMode>,
    pub(super) num_args: usize,
    pub(super) op: Opcode,
}

impl Into<Instruction> for i32 {
    fn into(self) -> Instruction {
        let op_num = self % 100;
        let op = Opcode::maybe_new(op_num).expect("Known opcode");
        let rem = (self - op_num) / 100;
        let digits: Vec<_> = digits(rem).into_iter().rev().collect();

        let num_args = match op {
            Opcode::Add => 3,
            Opcode::Mul => 3,
            Opcode::Inp => 1,
            Opcode::Out => 1,
            Opcode::Hlt => 0,
            Opcode::Jit => 2,
            Opcode::Jif => 2,
            Opcode::Clt => 3,
            Opcode::Ceq => 3,
        };

        let mut addr_modes = Vec::new();
        for i in 0..num_args {
            addr_modes
                .push(AddrMode::maybe_new(*digits.get(i).unwrap_or(&0)).expect("Valid opcode"));
        }

        Instruction {
            addr_modes: addr_modes,
            num_args: num_args,
            op: op,
        }
    }
}

#[derive(Debug)]
pub(super) enum Opcode {
    Add = 1,
    Mul = 2,
    Inp = 3,
    Out = 4,
    Jit = 5,
    Jif = 6,
    Clt = 7,
    Ceq = 8,
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
            5 => Some(Jit),
            6 => Some(Jif),
            7 => Some(Clt),
            8 => Some(Ceq),
            99 => Some(Hlt),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(super) enum AddrMode {
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

#[derive(Debug)]
pub(super) struct Argument {
    pub(super) src: Option<usize>,
    pub(super) v: i32,
}
