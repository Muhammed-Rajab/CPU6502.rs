/*
 *
 *
 *----------------
 * MEMORY LAYOUT |
 *----------------
 *
 * $0000-$00FF = Zero Page (256 bytes)
 * $0100-$01FF = Stack (second page, 256 bytes)
 * $0020-$FFF9 = General memory
 * $FFFA-$FFFB = NMI Vector
 * $FFFC-$FFFD = Reset Vector
 * $FFFE-$FFFF = IRQ/BRK Vector
 * */
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
            sp: 0xFD, // Reset value of SP
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
        /*
         * given,
         *      start = 0x1000
         *      len   = 10
         * the function outputs values from 0x1000..0x1009
         * [start, end)
         *
         * TODO: ASCII output
         */
        let aligned_start = start & !0x000F;
        let end = start
            .checked_add(len)
            .expect("hexdump range exceeds address space");
        let aligned_end = (end + 0x000F) & !0x000F;

        // print top row showing byte alignment
        print!("      ");
        for i in 0..=15 {
            print!("{:02x} ", i);
        }
        println!();

        // dump memory
        for addr in aligned_start..aligned_end {
            // print start if addr % 16 == 0
            if (addr & 0x000F) == 0 {
                print!("\n");
                print!("${:04x} ", addr);
            }

            // print '-- ' if addr < start
            if addr < start || addr >= end {
                print!("-- ");
            } else {
                // read and print the value if addr in [start, end)
                let byte = self.read(addr);
                print!("{:02x} ", byte);
            }
        }

        println!();
    }
}

fn main() {
    let mut cpu = Cpu6502::new();

    cpu.write(0x2222, 0xFF);
    cpu.write(0x2223, 0xaa);
    cpu.write(0x2320, 0xFF);

    cpu.hexdump(0x2222, 0x00ff);
}
