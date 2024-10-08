fn main() -> Result<(), String> {
    let mut machine = Machine::new();
    machine.memory.write(0, 0xff);
    machine.step()?;
    machine.step()?;
    Ok(())
}

// ---------------------------------------------------------------------//

#[allow(dead_code)]
enum Register {
    A,
    B,
    C,
    SP,
    PC,
    BP,
    FLAGS,
}

pub enum Op {
    Nop,
    Push(u8),
    PopReg(Register),
    AddStack,
}

trait Addressable {
    fn read(&self, addrs: u16) -> Option<u8>;
    fn write(&mut self, addrs: u16, value: u8) -> bool;

    // two bytes
    fn read2(&self, addr: u16) -> Option<u16> {
        if let Some(x0) = self.read(addr) {
            if let Some(x1) = self.read(addr + 1) {
                return Some((x0 as u16) | (x1 as u16) << 8);
            }
        }
        None
    }
    fn write2(&mut self, addr: u16, value: u16) -> bool {
        let lower = value & 0xff;
        let upper = (value & 0xff00) >> 8;
        self.write(addr, lower as u8) && self.write(addr + 1, upper as u8)
    }
    fn copy(&mut self, from: u16, to: u16, n: usize) -> bool {
        for i in 0..n {
            if let Some(x) = self.read(from + i as u16) {
                if !self.write(to + i as u16, x) {
                    return false;
                }
            } else {
                return false;
            }
        }
        return true;
    }
}

struct LinearMemory {
    bytes: Vec<u8>,
    size: usize,
}

impl LinearMemory {
    pub fn new(n: usize) -> Self {
        Self {
            bytes: vec![0; n],
            size: n,
        }
    }
}

impl Addressable for LinearMemory {
    fn read(&self, addr: u16) -> Option<u8> {
        if (addr as usize) < self.size {
            Some(self.bytes[addr as usize])
        } else {
            None
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> bool {
        if (addr as usize) < self.size {
            self.bytes[addr as usize] = value;
            true
        } else {
            false
        }
    }
}

struct Machine {
    registers: [u16; 8],
    pub memory: LinearMemory,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            registers: [0; 8],
            memory: LinearMemory::new(8 * 1024),
        }
    }

    pub fn step(&mut self) -> Result<(), String> {
        let pc = self.registers[Register::PC as usize];
        let instruction = self.memory.read2(pc).unwrap();

        self.registers[Register::PC as usize] = pc + 2;

        // instruction = [ 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0]
        //                      OPERATOR   | ARG(s)
        //                                 | 8 bit literal
        //                                 | REG1 | REG2
        //
        let op = (instruction & 0xff) as u8;
        match op {
            x if x == Op::Nop as u8 => Ok(()),
            _ => Err(format!("unknown operator 0x{:X}", op)),
        }
    }
}
