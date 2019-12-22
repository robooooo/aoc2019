use crate::{intcode::error::IntcodeErr, utils::digits};
use std::usize;

pub(super) struct Instruction {
    pub(super) addr_modes: Vec<AddrMode>,
    pub(super) num_args: i128,
    pub(super) op: Opcode,
}

impl Instruction {
    pub fn maybe_new(n: i128) -> Result<Self, IntcodeErr> {
        let op_num = n % 100;
        let op = match Opcode::maybe_new(op_num) {
            Some(op) => op,
            None => return Err(IntcodeErr::UnknownInstruction),
        };
        let rem = (n - op_num) / 100;
        let digits: Vec<_> = digits(rem as i32).into_iter().rev().collect();

        let num_args: i128 = match op {
            Opcode::Add => 3,
            Opcode::Mul => 3,
            Opcode::Inp => 1,
            Opcode::Out => 1,
            Opcode::Hlt => 0,
            Opcode::Jit => 2,
            Opcode::Jif => 2,
            Opcode::Clt => 3,
            Opcode::Ceq => 3,
            Opcode::Arb => 1,
        };

        let mut addr_modes = Vec::new();
        for i in 0..num_args {
            assert!(i <= usize::MAX as i128);
            let mode = match AddrMode::maybe_new(*digits.get(i as usize).unwrap_or(&0) as i128) {
                Some(mode) => mode,
                None => return Err(IntcodeErr::UnknownMode),
            };
            addr_modes.push(mode);
        }

        let res = Instruction {
            addr_modes: addr_modes,
            num_args: num_args,
            op: op,
        };
        Ok(res)
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
    Arb = 9,
    Hlt = 99,
}

impl Opcode {
    // Should be a better way to do this
    fn maybe_new(n: i128) -> Option<Opcode> {
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
            9 => Some(Arb),
            99 => Some(Hlt),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(super) enum AddrMode {
    Position = 0,
    Direct = 1,
    Relative = 2,
}

impl AddrMode {
    fn maybe_new(n: i128) -> Option<AddrMode> {
        use AddrMode::*;

        match n {
            0 => Some(Position),
            1 => Some(Direct),
            2 => Some(Relative),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub(super) struct Argument {
    pub(super) src: Option<i128>,
    pub(super) v: i128,
}
