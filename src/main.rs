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

        println!("0x{:04x}", start);
        println!("0x{:04x}", aligned_start);
        println!("0x{:04x}", end);
        println!("0x{:04x}", aligned_end);

        // print top row showing byte alignment
        print!("      ");
        for i in 0..=15 {
            print!("{:02x} ", i);
        }

        // print start address
        let mut counter = aligned_start;

        while counter != aligned_end {
            // print start if counter % 16 == 0
            if counter % 16 == 0 {
                print!("\n");
                print!("${:04x} ", counter);
            }

            // print '-- ' if counter < start
            if counter < start {
                print!("-- ");
            } else if counter > end {
                // print '-- ' if counter >= end (NOTE: check me)
                print!("-- ");
            } else {
                // read and print the value if counter in [start, end)
                let byte = self.read(counter);
                print!("{:02x} ", byte);
            }

            counter += 1;
        }
    }
}

fn main() {
    let mut cpu = Cpu6502::new();

    cpu.write(0x2222, 0xFF);
    cpu.write(0x2223, 0xaa);

    cpu.hexdump(0x2222, 0x00ff);
}
