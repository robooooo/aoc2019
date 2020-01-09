use crate::{
    intcode::{error::IntcodeErr, interpreter::Int},
    utils::digits,
};
use std::{ops::Deref, usize};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Instruction {
    pub(super) addr_modes: Vec<AddrMode>,
    pub(super) num_args: Int,
    pub(super) op: Opcode,
}

impl Instruction {
    pub fn maybe_new(n: Int) -> Result<Self, IntcodeErr> {
        let op_num = n % 100;
        let op = match Opcode::maybe_new(op_num) {
            Some(op) => op,
            None => return Err(IntcodeErr::UnknownInstruction(op_num)),
        };
        let rem = (n - op_num) / 100;
        let digits: Vec<_> = digits(rem as i32).into_iter().rev().collect();

        let num_args: Int = match op {
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
            assert!(i <= usize::MAX as Int);
            let mode = match AddrMode::maybe_new(*digits.get(i as usize).unwrap_or(&0) as Int) {
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
    fn maybe_new(n: Int) -> Option<Opcode> {
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(super) enum AddrMode {
    Position = 0,
    Direct = 1,
    Relative = 2,
}

impl AddrMode {
    fn maybe_new(n: Int) -> Option<AddrMode> {
        use AddrMode::*;

        match n {
            0 => Some(Position),
            1 => Some(Direct),
            2 => Some(Relative),
            _ => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(super) struct Argument<'a> {
    pub(super) reference: Option<&'a mut Int>,
    value: Int,
}

impl<'a> Argument<'a> {
    pub fn new(addr: &'a mut Int) -> Self {
        let v = *addr;
        Argument {
            reference: Some(addr),
            value: v,
        }
    }

    pub fn with_value(value: Int) -> Self {
        Argument {
            reference: None,
            value: value,
        }
    }

    pub fn reference(self) -> Option<&'a mut Int> {
        self.reference
    }

    // pub fn value(&self) -> Int {
    //     self.value
    // }
}

impl<'a> Deref for Argument<'a> {
    type Target = Int;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
