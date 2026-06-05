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
        let aligned_start = start & !0x000F;
        let end = start + len;
        let aligned_end = (end + 0x000F) & !0x000F;

        println!("0x{:04x}", aligned_start);
        println!("0x{:04x}", end);
        println!("0x{:04x}", aligned_end);
    }
}

fn main() {
    let mut cpu = Cpu6502::new();

    cpu.write(0x2222, 0xFF);
    cpu.write(0x2223, 0xaa);

    cpu.hexdump(0x2222, 0x22ff);
}
