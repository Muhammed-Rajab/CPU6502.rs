/*
 *
 *---------------
 * STATUS FLAGS |
 *---------------
 *
 * 7 6 5 4  3 2 1 0
 * - - - -  - - - -
 * N V 1 B  D I Z C
 *
 *----------------
 * MEMORY LAYOUT |
 *----------------
 *
 * $0000-$00FF = Zero Page (256 bytes)
 * $0100-$01FF = Stack (second page, 256 bytes)
 * $0200-$FFF9 = General memory
 * $FFFA-$FFFB = NMI Vector
 * $FFFC-$FFFD = Reset Vector
 * $FFFE-$FFFF = IRQ/BRK Vector
 *
 * For debugging purposes, our PC starts at $0600.
 * */

const START_PC_ADDRESS: u16 = 0x0600;

struct Cpu6502 {
    a: u8,               // A ccumulator
    x: u8,               // X register
    y: u8,               // Y register
    sp: u8,              // Stack pointer
    pc: u16,             // Program Counter
    status: u8,          // Status Flags
    memory: [u8; 65536], // Memory
}

enum Flag {
    Carry = 1,
}

const FLAG_CARRY: u8 = 1 << 0;
const FLAG_ZERO: u8 = 1 << 1;
const FLAG_INTERRUPT: u8 = 1 << 2;
const FLAG_DECIMAL: u8 = 1 << 3;
const FLAG_BREAK: u8 = 1 << 4;
const FLAG_UNUSED: u8 = 1 << 5;
const FLAG_OVERFLOW: u8 = 1 << 6;
const FLAG_NEGATIVE: u8 = 1 << 7;

impl Cpu6502 {
    fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            sp: 0xFD, // Reset value of SP
            pc: START_PC_ADDRESS,
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

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.read(self.pc);
        self.pc += 1;
        byte
    }

    fn set_flag(&mut self, flag: u8, value: bool) {
        if value {
            self.status |= flag;
        } else {
            self.status &= !flag;
        }
    }

    fn get_flag(&mut self, flag: u8) -> bool {
        (self.status & flag) != 0
    }

    fn step(&mut self) {
        let opcode = self.fetch_byte();

        match opcode {
            // LDA
            //
            // Immediate
            0xA9 => {
                let value = self.fetch_byte();
                self.lda(value);
            }
            _ => panic!("unknown opcode: {:02x}", opcode),
        };
    }

    //---------------------------------------------------------

    fn lda(&mut self, value: u8) {
        self.a = value;
        // update flags
    }

    //---------------------------------------------------------

    fn load_rom_from_memory(&mut self, rom: &[u8]) {
        let start = START_PC_ADDRESS as usize;
        let end = start + rom.len();

        if end > 65536 {
            panic!("rom too big");
        }

        self.memory[start..end].copy_from_slice(rom);
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
    let rom = [
        0xa9, 0x05, // LDA #5
        0xa9, 0x00, // LDA #0
    ];

    cpu.load_rom_from_memory(&rom);
    cpu.hexdump(START_PC_ADDRESS, 0x00ff);
}
