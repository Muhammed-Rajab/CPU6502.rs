struct Cpu6502 {
    a: u8,               // A ccumulator
    x: u8,               // X register
    y: u8,               // Y register
    sp: u8,              // Stack pointer
    pc: u16,             // Program Counter
    status: u8,          // Status Flags
    memory: [u8; 65536], // Memory
}

impl Cpu6502 {
    fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            pc: 0,
            status: 0,
            memory: [0; 65536], // Zero initialised array
        }
    }

    fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    fn hexdump(&self, start: u16, len: u16) {
        // TODO: output a hex dump of memory
    }
}

fn main() {
    let cpu = Cpu6502::new();
}
