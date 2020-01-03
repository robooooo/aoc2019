use crate::{
    intcode::{error::IntcodeErr, structs::*},
};
use std::{i128, iter, usize};

pub type Int = i128;

macro_rules! try_opt {
    ($ex:expr) => {
        match $ex {
            Some(e) => e,
            None => return None,
        }
    }
}

/// Fully evaluate the intcode instance `ic` to completion
/// Stops with an `Err(IntcodeErr::EvalNoArgs)` when it can't provide an argument
/// Stops with an `Err(E)` when the interpreter errors with code E
/// Otherwise returns a Vec<Int> of all the outputs before halting
pub fn eval<I>(ic: &mut Intcode, mut inp: I) -> Result<Vec<Int>, IntcodeErr>
where
    I: Iterator<Item = Int>,
{
    let mut out = Vec::new();
    loop {
        match ic.step() {
            State::Running => continue,
            State::Output(o) => out.push(o),
            State::Waiting => ic.input(inp.next().ok_or(IntcodeErr::EvalNoArgs)?),
            State::Halted => break,
            State::Error(e) => return Err(e),
        }
    }
    Ok(out)
}

/// Evaluate an intcode instance with all inputs as `inp`
/// Returns `Some(out)` on the first output
/// Will return `None` if it stops running before an output is produced
pub fn eval_once(ic: &mut Intcode, inp: Int) -> Result<Int, IntcodeErr> {
    match eval(ic, iter::once(inp)).map(|v| v.get(0).copied()) {
        Ok(None) => Err(IntcodeErr::EvalNoArgs),
        Ok(Some(i)) => Ok(i),
        Err(e) => Err(e),
    }
}

/// Like eval_once but uses an argument vector instead of a single arg
/// Will return `None` if the interpreter asks for too many inputs
pub fn eval_args(ic: &mut Intcode, argv: impl Iterator<Item = Int>) -> Result<Int, IntcodeErr> {
    match eval(ic, argv).map(|v| v.get(0).copied()) {
        Ok(None) => Err(IntcodeErr::EvalNoArgs),
        Ok(Some(i)) => Ok(i),
        Err(e) => Err(e),
    }
}

/// Like eval_once but returns
pub fn eval_all(ic: &mut Intcode, arg: Int) -> Result<Vec<Int>, IntcodeErr> {
    eval(ic, iter::once(arg))
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum State {
    Running,
    Waiting,
    Output(Int),
    Halted,
    Error(IntcodeErr),
}

pub struct Intcode {
    mem: Vec<Int>,
    ip: Int,
    base: Int,
    state: State,
    next_in: Option<Int>,
}

impl Intcode {
    pub fn new(mem: Vec<Int>) -> Self {
        Intcode {
            mem,
            ip: 0,
            base: 0,
            state: State::Running,
            next_in: None,
        }
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn mem(&self) -> &[Int] {
        &self.mem
    }

    pub fn input(&mut self, input: Int) {
        self.next_in = Some(input);
    }

    pub fn step(&mut self) -> State {
        if let State::Error(_) = self.state {
            return self.state;
        } else if self.state == State::Halted {
            return self.state;
        }

        assert!(self.ip <= usize::MAX as Int);
        let instr = match Instruction::maybe_new(self.mem[self.ip as usize]) {
            Ok(instr) => instr,
            Err(err) => {
                self.state = State::Error(err);
                return self.state;
            }
        };
        self.execute(&instr);

        if self.state != State::Waiting {
            self.ip += (instr.num_args + 1) as Int;
        }
        return self.state;
    }

    fn get(&mut self, addr: Int) -> Option<&mut Int> {
        let len = self.mem.len();
        assert!(addr <= usize::MAX as Int);
        let addr = addr as usize;

        if len <= addr {
            self.mem.extend(iter::repeat(0).take(addr - len + 1));
        }

        match self.mem.get_mut(addr) {
            opt @ Some(_) => opt,
            None => {
                self.state = State::Error(IntcodeErr::NegativeAccess);
                None
            }
        } 
    }

    fn arg(&mut self, mode: AddrMode, idx: Int) -> Option<Argument> {
        let param = *try_opt!(self.get(self.ip + idx + 1));
        let idx = match mode {
            AddrMode::Position => param,
            AddrMode::Relative => self.base + param,
            AddrMode::Direct => return Some(Argument::with_value(param)),
        };
        self.get(idx).map(Argument::new)
    }

    fn execute<'a>(&'a mut self, instr: &Instruction) {
        macro_rules! addr {
            ( $arg:expr ) => {{
                let i = $arg as usize;
                let arg = match self.arg(instr.addr_modes[i], i as Int) {
                    Some(arg) => arg,
                    None => return,
                };
                match arg.reference() {
                    Some(reference) => reference,
                    None => {
                        self.state = State::Error(IntcodeErr::WriteDirect);
                        return;
                    }
                }
            }};
        }

        macro_rules! arg {
            ( $arg:expr ) => {{
                let i = $arg as usize;
                match self.arg(instr.addr_modes[i], i as Int) {
                    Some(v) => *v,
                    None => {
                        self.state = State::Error(IntcodeErr::WriteDirect);
                        return;
                    }
                }
            }};
        }

        self.state = State::Running;
        match instr.op {
            Opcode::Add => *addr!(2) = arg!(0) + arg!(1),
            Opcode::Mul => *addr!(2) = arg!(0) * arg!(1),

            Opcode::Inp => match self.next_in {
                Some(input) => {
                    *addr!(0) = input;
                    self.next_in = None;
                    self.state = State::Running;
                }
                None => self.state = State::Waiting,
            },

            Opcode::Out => self.state = State::Output(arg!(0)),
            Opcode::Hlt => self.state = State::Halted,

            Opcode::Jit => {
                if arg!(0) != 0 {
                    self.ip = arg!(1) - instr.num_args - 1
                }
            }
            Opcode::Jif => {
                if arg!(0) == 0 {
                    self.ip = arg!(1) - instr.num_args - 1
                }
            }

            Opcode::Clt => *addr!(2) = (arg!(0) < arg!(1)) as Int,
            Opcode::Ceq => *addr!(2) = (arg!(0) == arg!(1)) as Int,

            Opcode::Arb => self.base += arg!(0),
        }
    }
}
