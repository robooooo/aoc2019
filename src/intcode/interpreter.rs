use crate::intcode::{error::IntcodeErr, structs::*};

/// Evaluate an intcode instance with all inputs as `inp`
/// Returns `Some(out)` on the first output
/// Will return `None` if it stops running before an output is produced
pub fn eval(ic: &mut Intcode, inp: i32) -> Option<i32> {
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
pub fn eval_args(ic: &mut Intcode, argv: &Vec<i32>) -> Option<i32> {
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

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum State {
    Running,
    Waiting,
    Output(i32),
    Halted,
    Error(IntcodeErr),
}

pub struct Intcode {
    mem: Vec<i32>,
    ip: usize,
    state: State,
    next_in: Option<i32>,
}

impl Intcode {
    pub fn new(mem: Vec<i32>) -> Self {
        Intcode {
            mem,
            ip: 0,
            state: State::Running,
            next_in: None,
        }
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn mem(&self) -> &[i32] {
        &self.mem
    }

    pub fn input(&mut self, input: i32) {
        self.next_in = Some(input);
    }

    pub fn step(&mut self) -> State {
        if let State::Error(_) = self.state {
            return self.state;
        } else if self.state == State::Halted {
            return self.state;
        }

        let instr = self.mem[self.ip].into();
        self.execute(&instr);

        if self.state != State::Waiting {
            self.ip += instr.num_args + 1;
        }
        return self.state;
    }

    fn write(&mut self, arg: &Argument, v: i32) {
        let addr = match arg.src {
            Some(addr) => addr,
            None => {
                self.state = State::Error(IntcodeErr::WriteDirect);
                return;
            }
        };
        match self.mem.get_mut(addr) {
            Some(loc) => *loc = v,
            None => self.state = State::Error(IntcodeErr::WriteOob),
        };
    }

    fn arg(&mut self, mode: AddrMode, arg_idx: usize) -> Argument {
        let arg = self.address(mode, self.mem[self.ip + 1 + arg_idx]);
        arg
    }

    fn address(&mut self, mode: AddrMode, value: i32) -> Argument {
        match mode {
            AddrMode::Position => Argument {
                src: Some(value as usize),
                v: match self.mem.get(value as usize) {
                    Some(v) => *v,
                    None => {
                        self.state = State::Error(IntcodeErr::ReadOob);
                        0
                    }
                },
            },
            AddrMode::Direct => Argument {
                src: None,
                v: value,
            },
        }
    }

    fn execute<'a>(&'a mut self, instr: &Instruction) {
        let mut argv = Vec::with_capacity(instr.num_args);
        for i in 0..instr.num_args {
            argv.push(self.arg(instr.addr_modes[i], i));
        }

        self.state = State::Running;
        match instr.op {
            Opcode::Add => self.write(&argv[2], argv[0].v + argv[1].v),
            Opcode::Mul => self.write(&argv[2], argv[0].v * argv[1].v),

            Opcode::Inp => match self.next_in {
                Some(input) => {
                    self.write(&argv[0], input);
                    self.next_in = None;
                    self.state = State::Running;
                }
                None => self.state = State::Waiting,
            },

            Opcode::Out => self.state = State::Output(argv[0].v),
            Opcode::Hlt => self.state = State::Halted,

            Opcode::Jit => {
                if argv[0].v != 0 {
                    self.ip = argv[1].v as usize - instr.num_args - 1
                }
            }
            Opcode::Jif => {
                if argv[0].v == 0 {
                    self.ip = argv[1].v as usize - instr.num_args - 1
                }
            }

            Opcode::Clt => self.write(&argv[2], (argv[0].v < argv[1].v) as i32),
            Opcode::Ceq => self.write(&argv[2], (argv[0].v == argv[1].v) as i32),
        }
    }
}
