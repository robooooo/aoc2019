use crate::utils::digits;
use log::{debug, trace};

static ADDR_ERR: &'static str = "addressing mode of write param to be position mode";

pub struct Intcode {
    input_req: i32,
    mem: Vec<i32>,
    ip: usize,
    out: Option<i32>,
    running: bool,
}

impl Intcode {
    pub fn new(mem: Vec<i32>, input: i32) -> Self {
        Intcode {
            input_req: input,
            mem: mem,
            ip: 0,
            out: None,
            running: true,
        }
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn out(&self) -> Option<i32> {
        self.out
    }

    fn write(&mut self, arg: &Argument, v: i32) {
        let addr = arg.src.expect(ADDR_ERR);
        trace!("Wrote {} to {}", v, addr);
        self.mem[addr] = v;
    }

    fn arg(&self, mode: AddrMode, arg_idx: usize) -> Argument {
        trace!("Getting arg {} with addressing mode {:?}", arg_idx, mode);
        let arg = self.address(mode, self.mem[self.ip + 1 + arg_idx]);
        trace!("Got {} from {:?}", arg.v, arg.src);
        arg
    }

    fn address(&self, mode: AddrMode, value: i32) -> Argument {
        match mode {
            AddrMode::Position => Argument {
                src: Some(value as usize),
                v: *self.mem.get(value as usize).unwrap_or(&0),
            },
            AddrMode::Direct => Argument {
                src: None,
                v: value,
            },
        }
    }

    pub fn step(&mut self) {
        if !self.running {
            return;
        }
        let instr = self.mem[self.ip].into();
        debug!("IP is {}, points to {}", self.ip, self.mem[self.ip]);
        self.execute(&instr);
        self.ip += instr.num_args + 1;
    }

    fn execute<'a>(&'a mut self, instr: &Instruction) {
        let mut argv = Vec::with_capacity(instr.num_args);
        trace!("Performing instruction {:?}", instr.op);
        for i in 0..instr.num_args {
            argv.push(self.arg(instr.addr_modes[i], i));
        }

        self.out = None;
        match instr.op {
            Opcode::Add => self.write(&argv[2], argv[0].v + argv[1].v),
            Opcode::Mul => self.write(&argv[2], argv[0].v * argv[1].v),
            Opcode::Inp => self.write(&argv[0], self.input_req),
            Opcode::Out => self.out = Some(argv[0].v),
            Opcode::Hlt => self.running = false,
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

struct Instruction {
    addr_modes: Vec<AddrMode>,
    num_args: usize,
    op: Opcode,
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
enum Opcode {
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

#[derive(Debug)]
struct Argument {
    src: Option<usize>,
    v: i32,
}
