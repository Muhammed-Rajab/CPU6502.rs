/*
 *
 *-----------------
 * SPECIFICATIONS |
 *-----------------
 *
 * 8-bit little endian
 *
 *---------------
 * STATUS FLAGS |
 *---------------
 *
 * 7 6 5 4  3 2 1 0
 * - - - -  - - - -
 * N V 1 B  D I Z C
 *
 * Carry (C) -> Set if last operation an overflow from
 *              bit 7 of the result (like in `ADC`) or an
 *              underflow (like in `SBC`).
 *
 *              Arithmetic, comparison, and logical shifts.
 *
 *              Explicitly set using the `SEC` instruction and
 *              cleared with `CLC` instruction.
 *
 * Zero (Z) -> Set if the result of last operation was zero.
 *
 * Interrupt Disable (I) -> Set if `SEI` instruction was executed.
 *                          When set, the processor will not
 *                          respond to interrupts from devices until
 *                          it is cleared by `CLI` instruction.
 *
 * Decimal Mode (D) -> When set, the processor obeys Binary Coded Decimal
 *                      arithmetic during addition and subtraction (not sure
 *                      what that means, lol). Can be set using `SED` and
 *                      cleared with `CLD`.
 *
 * Break (B) -> Set when BRK instruction has been executed and an interrupt
 *              has been generated to process it.
 *
 * Overflow (V) -> Set during arithmetic operations if the results has yielded
 *                  an invalid 2's complement result (eg: adding positives and
 *                  end up getting negative, 64 + 64 = -128
 *
 *                  Can't trigger overflow if two numbers you are adding have
 *                  different signs.
 *
 *                  Carry is for unsigned.
 *                  Overflow is for signed.
 *
 * Negative (N) -> Set if the result of the last operation had bit 7 set to one.
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
 * SP starts at $FF, and decreases when pushed into and vice versa.
 * SP wraps since it's an 8-bit register. PHA at SP=$00 stores value of A to $00 and decreases
 * it by one, wrapping it, resulting in SP=$FF.
 * For debugging purposes, our PC starts at $0600.
 * */

mod cpu_tests;
mod flags;
mod instructions;
mod memory;

use flags::Flag;

pub const START_PC_ADDRESS: u16 = 0x0600;

pub struct Cpu6502 {
    a: u8,               // A ccumulator
    x: u8,               // X register
    y: u8,               // Y register
    sp: u8,              // Stack pointer
    pc: u16,             // Program Counter
    status: u8,          // Status Flags
    memory: [u8; 65536], // Memory
}

//-----------------------------------------------
// CONSTRUCTOR                                  |
//-----------------------------------------------

impl Cpu6502 {
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            sp: 0xFF,                   // Reset value of SP
            pc: START_PC_ADDRESS,       // Should be loaded from Reset Vector
            status: Flag::Unused as u8, // By default, it's on.
            memory: [0; 65536],         // Zero initialised array
        }
    }
}

//-----------------------------------------------
// OPERATION                                    |
//-----------------------------------------------

impl Cpu6502 {
    pub fn step(&mut self) {
        let opcode = self.fetch_byte();

        println!("PC=${:04X}, OPCODE=${:02X}", self.pc, opcode);

        match opcode {
            //------
            // LDA |
            //------

            // Immediate
            0xA9 => {
                let value = self.fetch_byte();
                self.lda(value);
            }

            //------
            // LDX |
            //------

            // Immediate
            0xA2 => {
                let value = self.fetch_byte();
                self.ldx(value);
            }

            //------
            // LDY |
            //------

            // Immediate
            0xA0 => {
                let value = self.fetch_byte();
                self.ldy(value);
            }

            //------
            // STA |
            //------

            // Absolute
            0x8D => {
                // $1234 in memory -> $34 $12
                let low = self.fetch_byte() as u16;
                let high = self.fetch_byte() as u16;
                let addr = (high << 8) | low;
                self.sta(addr);
            }

            // Zero Page
            0x85 => {
                // $00-$ff
                let addr = self.fetch_byte() as u16;
                self.sta(addr);
            }

            //------
            // STX |
            //------

            // Absolute
            0x8E => {
                // $1234 in memory -> $34 $12
                let low = self.fetch_byte() as u16;
                let high = self.fetch_byte() as u16;
                let addr = (high << 8) | low;
                self.stx(addr);
            }

            // Zero Page
            0x86 => {
                // $00-$ff
                let addr = self.fetch_byte() as u16;
                self.stx(addr);
            }

            //------
            // STY |
            //------

            // Absolute
            0x8C => {
                // $1234 in memory -> $34 $12
                let low = self.fetch_byte() as u16;
                let high = self.fetch_byte() as u16;
                let addr = (high << 8) | low;
                self.sty(addr);
            }

            // Zero Page
            0x84 => {
                // $00-$ff
                let addr = self.fetch_byte() as u16;
                self.sty(addr);
            }

            //------
            // TAX |
            //------

            // Implied
            0xAA => {
                self.tax();
            }

            //------
            // TAY |
            //------

            // Implied
            0xA8 => {
                self.tay();
            }

            //------
            // TXA |
            //------

            // Implied
            0x8A => {
                self.txa();
            }

            //------
            // TYA |
            //------

            // Implied
            0x98 => {
                self.tya();
            }

            //------
            // TSX |
            //------

            // Implied
            0xBA => {
                self.tsx();
            }

            //------
            // TXS |
            //------

            // Implied
            0x9A => {
                self.txs();
            }

            //------
            // PHA |
            //------

            // Implied
            0x48 => {
                self.pha();
            }

            //------
            // PLA |
            //------

            // Implied
            0x68 => {
                self.pla();
            }

            //------
            // PHP |
            //------

            // Implied
            0x08 => {
                self.php();
            }

            //------
            // PLP |
            //------

            // Implied
            0x28 => {
                self.plp();
            }

            // Default
            _ => panic!("unknown opcode: {:02x}", opcode),
        };
    }
}

//-----------------------------------------------
// LOAD                                         |
//-----------------------------------------------

impl Cpu6502 {
    pub fn load_rom_from_memory(&mut self, rom: &[u8]) {
        let start = START_PC_ADDRESS as usize;
        let end = start + rom.len();

        if end > 65536 {
            panic!("rom too big");
        }

        self.memory[start..end].copy_from_slice(rom);
    }
}

//-----------------------------------------------
// DEBUG                                        |
//-----------------------------------------------

impl Cpu6502 {
    pub fn hexdump(&self, start: u16, len: u16) {
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
