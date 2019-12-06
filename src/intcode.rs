use crate::utils::{self, digits};

pub struct Intcode {
    mem: Vec<i32>,
    ip: usize,
    out: Option<i32>,
    running: bool,
}

impl Intcode {
    pub fn new(mem: Vec<i32>) -> Self {
        Intcode { mem: mem, ip: 0, out: None, running: true }
    }

    fn address(&self, mode: AddrMode, value: i32) -> i32 {
        match mode {
            AddrMode::Position => self.mem[value as usize],
            AddrMode::Direct => value,
        }
    }

    pub fn step(&mut self) {
        if !self.running {
            return;
        }
        let instr = self.mem[self.ip].into();
        self.execute(&instr);
        self.ip += instr.num_args + 1;
    }

    fn execute<'a>(&'a mut self, instr: &Instruction) {
        
        let mut argv = Vec::with_capacity(instr.num_args);
        for i in 0..instr.num_args {
            argv.push(*self.mem.get(self.ip + 1 + i).unwrap_or(&0));
        }

        match instr.op {
            Opcode::Add => self.mem[argv[2] as usize] = argv[0] + argv[1],
            Opcode::Mul => self.mem[argv[2] as usize] = argv[0] + argv[1],
            Opcode::Inp => self.mem[argv[2] as usize] = 0,
            Opcode::Out => self.out = Some(argv[0]),
            Opcode::Hlt => self.running = false,
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
            num_args: num_args,
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
