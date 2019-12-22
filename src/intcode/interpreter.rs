use crate::intcode::{error::IntcodeErr, structs::*};
use std::{usize, i128};

/// Evaluate an intcode instance with all inputs as `inp`
/// Returns `Some(out)` on the first output
/// Will return `None` if it stops running before an output is produced
pub fn eval(ic: &mut Intcode, inp: i128) -> Option<i128> {
    loop {
        match ic.step() {
            State::Running => continue,
            State::Output(out) => return Some(out),
            State::Waiting => ic.input(inp),
            _ => return None,
        }
    }
}

/// Like eval but uses an argument vector instead of a single arg
/// Will return `None` if the interpreter asks for too many inputs
pub fn eval_args(ic: &mut Intcode, argv: &Vec<i128>) -> Option<i128> {
    let mut inps = argv.iter();
    loop {
        match ic.step() {
            State::Running => continue,
            State::Output(out) => return Some(out),
            State::Waiting => match inps.next() {
                Some(&inp) => ic.input(inp),
                None => return None,
            },
            _ => return None,
        }
    }
}

pub fn eval_all(ic: &mut Intcode, arg: i128) -> Option<Vec<i128>> {
    let mut out = Vec::new();
    loop {
        match ic.step() {
            State::Running => continue,
            State::Output(x) => out.push(x),
            State::Waiting => ic.input(arg),
            State::Halted => break,
            _ => return None,
        }
    }
    Some(out)
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum State {
    Running,
    Waiting,
    Output(i128),
    Halted,
    Error(IntcodeErr),
}

pub struct Intcode {
    mem: Vec<i128>,
    ip: i128,
    base: i128,
    state: State,
    next_in: Option<i128>,
}

impl Intcode {
    pub fn new(mem: Vec<i128>) -> Self {
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

    pub fn mem(&self) -> &[i128] {
        &self.mem
    }

    pub fn input(&mut self, input: i128) {
        self.next_in = Some(input);
    }

    pub fn step(&mut self) -> State {
        if let State::Error(_) = self.state {
            return self.state;
        } else if self.state == State::Halted {
            return self.state;
        }

        assert!(self.ip <= usize::MAX as i128);
        let instr = match Instruction::maybe_new(self.mem[self.ip as usize]) {
            Ok(instr) => instr,
            Err(err) => {
                self.state = State::Error(err);
                return self.state;
            }
        };
        self.execute(&instr);


        if self.state != State::Waiting {
            self.ip += (instr.num_args + 1) as i128;
        }
        return self.state;
    }

    fn get(&mut self, addr: i128) -> &mut i128 {
        let len = self.mem.len();
        assert!(addr <= usize::MAX as i128);
        let addr = addr as usize;
        if len < addr {
            self.mem.reserve(addr - len + 1);
            for _ in len..=addr {
                self.mem.push(0);
            }
        }

        self.mem.get_mut(addr).unwrap()
    }

    fn write_arg(&mut self, arg: &Argument, v: i128) {
        let addr = match arg.src {
            Some(addr) => addr,
            None => {
                self.state = State::Error(IntcodeErr::WriteDirect);
                return;
            }
        };
        *self.get(addr) = v;
    }

    fn arg(&mut self, mode: AddrMode, arg_idx: i128) -> Argument {
        let v = *self.get(self.ip + 1 + arg_idx);
        let arg = self.address(mode, v);
        arg
    }

    fn address(&mut self, mode: AddrMode, value: i128) -> Argument {
        match mode {
            AddrMode::Position => Argument {
                src: Some(value),
                v: *self.get(value),
            },
            AddrMode::Direct => Argument {
                src: None,
                v: value,
            },
            AddrMode::Relative => Argument {
                src: Some(self.base + value),
                v: *self.get(value),
            }
        }
    }

    fn execute<'a>(&'a mut self, instr: &Instruction) {
        let mut argv = Vec::with_capacity(instr.num_args as usize);
        for i in 0..instr.num_args {
            let i = i as usize;
            argv.push(self.arg(instr.addr_modes[i], i as i128));
        }

        self.state = State::Running;
        match instr.op {
            Opcode::Add => self.write_arg(&argv[2], argv[0].v + argv[1].v),
            Opcode::Mul => self.write_arg(&argv[2], argv[0].v * argv[1].v),

            Opcode::Inp => match self.next_in {
                Some(input) => {
                    self.write_arg(&argv[0], input);
                    self.next_in = None;
                    self.state = State::Running;
                }
                None => self.state = State::Waiting,
            },

            Opcode::Out => self.state = State::Output(argv[0].v),
            Opcode::Hlt => self.state = State::Halted,

            Opcode::Jit => {
                if argv[0].v != 0 {
                    self.ip = argv[1].v - instr.num_args - 1
                }
            }
            Opcode::Jif => {
                if argv[0].v == 0 {
                    self.ip = argv[1].v - instr.num_args - 1
                }
            }

            Opcode::Clt => self.write_arg(&argv[2], (argv[0].v < argv[1].v) as i128),
            Opcode::Ceq => self.write_arg(&argv[2], (argv[0].v == argv[1].v) as i128),

            Opcode::Arb => self.base += argv[0].v,
        }
    }
}
