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

#[repr(u8)]
#[derive(Copy, Clone)]
enum Flag {
    Carry = 1 << 0,
    Zero = 1 << 1,
    Interrupt = 1 << 2,
    Decimal = 1 << 3,
    Break = 1 << 4,
    Unused = 1 << 5,
    Overflow = 1 << 6,
    Negative = 1 << 7,
}

//-----------------------------------------------
// CONSTRUCTOR                                  |
//-----------------------------------------------

impl Cpu6502 {
    fn new() -> Self {
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
// MEMORY                                       |
//-----------------------------------------------

impl Cpu6502 {
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

    fn peek_stack(&mut self) -> u8 {
        let top_sp = self.sp.wrapping_add(1);
        let addr = 0x0100u16 | top_sp as u16;
        self.read(addr)
    }

    fn push_to_stack(&mut self, value: u8) {
        let addr = 0x0100u16 | (self.sp as u16);
        self.write(addr, value);
        self.sp -= 1;
    }

    fn pull_from_stack(&mut self) -> u8 {
        self.sp += 1;
        let addr = 0x0100u16 | (self.sp as u16);
        self.read(addr)
    }
}

//-----------------------------------------------
// FLAGS                                        |
//-----------------------------------------------

impl Cpu6502 {
    fn set_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.status |= flag as u8;
        } else {
            self.status &= !(flag as u8);
        }

        // Always set
        self.status |= Flag::Unused as u8;
    }

    fn get_flag(&self, flag: Flag) -> bool {
        (self.status & (flag as u8)) != 0
    }

    fn update_zn(&mut self, value: u8) {
        self.set_flag(Flag::Zero, value == 0);
        self.set_flag(Flag::Negative, (value & 0x80) != 0);
    }
}

//-----------------------------------------------
// INSTRUCTIONS                                 |
//-----------------------------------------------

impl Cpu6502 {
    /*
     * Loads a byte of memory into accumulator.
     * Sets the N and Z flags as appropriate.
     */
    fn lda(&mut self, value: u8) {
        self.a = value;
        self.update_zn(self.a);
    }

    /*
     * Loads a byte of memory into X.
     * Sets the N and Z flags as appropriate.
     */
    fn ldx(&mut self, value: u8) {
        self.x = value;
        self.update_zn(self.x);
    }

    /*
     * Loads a byte of memory into Y.
     * Sets the N and Z flags as appropriate.
     */
    fn ldy(&mut self, value: u8) {
        self.y = value;
        self.update_zn(self.y);
    }

    /*
     * Stores the contents of A to memory[addr]
     * No flags affected.
     */
    fn sta(&mut self, addr: u16) {
        self.memory[addr as usize] = self.a;
    }

    /*
     * Stores the contents of X to memory[addr]
     * No flags affected.
     */
    fn stx(&mut self, addr: u16) {
        self.memory[addr as usize] = self.x;
    }

    /*
     * Stores the contents of Y to memory[addr]
     * No flags affected.
     */
    fn sty(&mut self, addr: u16) {
        self.memory[addr as usize] = self.y;
    }

    /*
     * Copies the contents of A to X
     * Sets Z and N flag appropriately.
     */
    fn tax(&mut self) {
        self.x = self.a;
        self.update_zn(self.x);
    }

    /*
     * Copies the contents of A to Y
     * Sets Z and N flag appropriately.
     */
    fn tay(&mut self) {
        self.y = self.a;
        self.update_zn(self.y);
    }

    /*
     * Copies the contents of X to A
     * Sets Z and N flag appropriately.
     */
    fn txa(&mut self) {
        self.a = self.x;
        self.update_zn(self.a);
    }

    /*
     * Copies the contents of Y to A
     * Sets Z and N flag appropriately.
     */
    fn tya(&mut self) {
        self.a = self.y;
        self.update_zn(self.a);
    }

    /*
     * Copies the contents of SP to X
     * Sets Z and N flag appropriately.
     */
    fn tsx(&mut self) {
        self.x = self.sp;
        self.update_zn(self.x);
    }

    /*
     * Copies the contents of X to SP
     * No flags affected.
     */
    fn txs(&mut self) {
        self.sp = self.x;
    }

    /*
     * Pushes a copy of A on to the stack
     * No flags affected.
     * SP decreased
     */
    fn pha(&mut self) {
        self.push_to_stack(self.a);
    }

    /*
     * Pulls 8-bit value from stack and into A.
     * Sets Z and N flags appropriately.
     * SP increases
     */
    fn pla(&mut self) {
        self.a = self.pull_from_stack();
        self.update_zn(self.a);
    }

    /*
     * Pushes a copy of processor status on to the stack
     * No flags affected.
     * SP decreased
     */
    fn php(&mut self) {
        self.push_to_stack(self.status);
    }

    /*
     * Pulls 8-bit value from stack and into processor status flags.
     * The flag will take new state determined by the value.
     * SP increases
     */
    fn plp(&mut self) {
        self.status = self.pull_from_stack();
    }
}

//-----------------------------------------------
// OPERATION                                    |
//-----------------------------------------------

impl Cpu6502 {
    fn step(&mut self) {
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
    fn load_rom_from_memory(&mut self, rom: &[u8]) {
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

//-----------------------------------------------
// TESTS                                        |
//-----------------------------------------------

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn flag_set_clear_test() {
        let mut cpu = Cpu6502::new();

        // by default, unused flag must be set
        assert!(cpu.get_flag(Flag::Unused));

        cpu.set_flag(Flag::Carry, true);
        cpu.set_flag(Flag::Zero, true);
        cpu.set_flag(Flag::Interrupt, true);
        cpu.set_flag(Flag::Decimal, true);
        cpu.set_flag(Flag::Break, true);
        cpu.set_flag(Flag::Overflow, true);
        cpu.set_flag(Flag::Negative, true);

        // all flags are set now
        assert_eq!(cpu.status, 0xFF);

        cpu.set_flag(Flag::Carry, false);
        cpu.set_flag(Flag::Zero, false);
        cpu.set_flag(Flag::Interrupt, false);
        cpu.set_flag(Flag::Decimal, false);
        cpu.set_flag(Flag::Break, false);
        cpu.set_flag(Flag::Unused, false); // has no effect
        cpu.set_flag(Flag::Overflow, false);
        cpu.set_flag(Flag::Negative, false);

        // all flags are cleared except unused
        assert_eq!(cpu.status, 0x20);
    }

    #[test]
    fn lda_test() {
        let mut cpu = Cpu6502::new();

        // Immediate Mode
        let immediate_rom = [
            0xA9u8, 0x10, // LDA #$10
            0xA9, 0x00, // LDA #$00
            0xA9, 0xFF, // LDA #$FF
        ];

        cpu.load_rom_from_memory(&immediate_rom);

        // 1st LDA
        cpu.step();
        assert_eq!(0x10, cpu.a);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));

        // 2nd LDA
        cpu.step();
        assert_eq!(0x00, cpu.a);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(true, cpu.get_flag(Flag::Zero));

        // 3rd LDA
        cpu.step();
        assert_eq!(0xFF, cpu.a);
        assert_eq!(true, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));
    }

    #[test]
    fn ldx_test() {
        let mut cpu = Cpu6502::new();

        // Immediate Mode
        let immediate_rom = [
            0xA2u8, 0x10, // LDX #$10
            0xA2, 0x00, // LDX #$00
            0xA2, 0xFF, // LDX #$FF
        ];

        cpu.load_rom_from_memory(&immediate_rom);

        // 1st LDX
        cpu.step();
        assert_eq!(0x10, cpu.x);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));

        // 2nd LDX
        cpu.step();
        assert_eq!(0x00, cpu.x);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(true, cpu.get_flag(Flag::Zero));

        // 3rd LDX
        cpu.step();
        assert_eq!(0xFF, cpu.x);
        assert_eq!(true, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));
    }

    #[test]
    fn ldy_test() {
        let mut cpu = Cpu6502::new();

        // Immediate Mode
        let immediate_rom = [
            0xA0u8, 0x10, // LDY #$10
            0xA0, 0x00, // LDY #$00
            0xA0, 0xFF, // LDY #$FF
        ];

        cpu.load_rom_from_memory(&immediate_rom);

        // 1st LDY
        cpu.step();
        assert_eq!(0x10, cpu.y);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));

        // 2nd LDY
        cpu.step();
        assert_eq!(0x00, cpu.y);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(true, cpu.get_flag(Flag::Zero));

        // 3rd LDY
        cpu.step();
        assert_eq!(0xFF, cpu.y);
        assert_eq!(true, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));
    }

    #[test]
    fn sta_test() {
        // Absolute Mode
        let mut cpu = Cpu6502::new();
        let absolute_rom = [
            0xA9, 0x42, //  LDA #$42
            0x8Du8, 0x34, 0x12, // STA $1234
        ];

        cpu.load_rom_from_memory(&absolute_rom);

        // 1st STA
        cpu.step();
        cpu.step();
        assert_eq!(cpu.read(0x1234), 0x42);

        // Zero Page Mode
        let mut cpu = Cpu6502::new();
        let zero_page_rom = [
            0xA9, 0x42, //  LDA #$42
            0x85u8, 0xff, // STA $ff
        ];

        cpu.load_rom_from_memory(&zero_page_rom);

        // 1st STA
        cpu.step();
        cpu.step();
        assert_eq!(cpu.read(0xff), 0x42);
    }

    #[test]
    fn stx_test() {
        // Absolute Mode
        let mut cpu = Cpu6502::new();
        let absolute_rom = [
            0xA2, 0x42, //  LDX #$42
            0x8Eu8, 0x34, 0x12, // STX $1234
        ];

        cpu.load_rom_from_memory(&absolute_rom);

        // 1st STX
        cpu.step();
        cpu.step();
        assert_eq!(cpu.read(0x1234), 0x42);

        // Zero Page Mode
        let mut cpu = Cpu6502::new();
        let zero_page_rom = [
            0xA2, 0x42, //  LDX #$42
            0x86u8, 0xff, // STX $ff
        ];

        cpu.load_rom_from_memory(&zero_page_rom);

        // 1st STX
        cpu.step();
        cpu.step();
        assert_eq!(cpu.read(0xff), 0x42);
    }

    #[test]
    fn sty_test() {
        // Absolute Mode
        let mut cpu = Cpu6502::new();
        let absolute_rom = [
            0xA0, 0x42, //  LDY #$42
            0x8Cu8, 0x34, 0x12, // STY $1234
        ];

        cpu.load_rom_from_memory(&absolute_rom);

        // 1st STY
        cpu.step();
        cpu.step();
        assert_eq!(cpu.read(0x1234), 0x42);

        // Zero Page Mode
        let mut cpu = Cpu6502::new();
        let zero_page_rom = [
            0xA0, 0x42, //  LDY #$42
            0x84u8, 0xff, // STY $ff
        ];

        cpu.load_rom_from_memory(&zero_page_rom);

        // 1st STX
        cpu.step();
        cpu.step();
        assert_eq!(cpu.read(0xff), 0x42);
    }

    #[test]
    fn tax_test() {
        let mut cpu = Cpu6502::new();
        let implied_rom = [
            0xA9u8, 0x0a, // LDA #$0a
            0xAA, // TAX
            0xA9u8, 0x00, // LDA #$00
            0xAA, // TAX
            0xA9u8, 0xff, // LDA #$ff
            0xAA, // TAX
        ];
        cpu.load_rom_from_memory(&implied_rom);

        cpu.step(); // LDA #$0a
        cpu.step(); // TAX
        assert_eq!(cpu.a, cpu.x);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));

        cpu.step(); // LDA #$00
        cpu.step(); // TAX
        assert_eq!(cpu.a, cpu.x);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(true, cpu.get_flag(Flag::Zero));

        cpu.step(); // LDA #$ff
        cpu.step(); // TAX
        assert_eq!(cpu.a, cpu.x);
        assert_eq!(true, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));
    }

    #[test]
    fn tay_test() {
        let mut cpu = Cpu6502::new();
        let implied_rom = [
            0xA9u8, 0x0a, // LDA #$0a
            0xA8, // TAY
            0xA9u8, 0x00, // LDA #$00
            0xA8, // TAY
            0xA9u8, 0xff, // LDA #$ff
            0xA8, // TAY
        ];
        cpu.load_rom_from_memory(&implied_rom);

        cpu.step(); // LDA #$0a
        cpu.step(); // TAY
        assert_eq!(cpu.a, cpu.y);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));

        cpu.step(); // LDA #$00
        cpu.step(); // TAY
        assert_eq!(cpu.a, cpu.y);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(true, cpu.get_flag(Flag::Zero));

        cpu.step(); // LDA #$ff
        cpu.step(); // TAY
        assert_eq!(cpu.a, cpu.y);
        assert_eq!(true, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));
    }

    #[test]
    fn txa_test() {
        let mut cpu = Cpu6502::new();
        let implied_rom = [
            0xA2, 0x0a, // LDX #$0a
            0x8a, // TXA
            0xA2, 0x00, // LDX #$00
            0x8a, // TXA
            0xA2, 0xff, // LDX #$ff
            0x8a, // TXA
        ];
        cpu.load_rom_from_memory(&implied_rom);

        cpu.step(); // LDX #$0a
        cpu.step(); // TXA
        assert_eq!(cpu.x, cpu.a);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));

        cpu.step(); // LDX #$00
        cpu.step(); // TXA
        assert_eq!(cpu.x, cpu.a);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(true, cpu.get_flag(Flag::Zero));

        cpu.step(); // LDX #$ff
        cpu.step(); // TXA
        assert_eq!(cpu.x, cpu.a);
        assert_eq!(true, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));
    }

    #[test]
    fn tya_test() {
        let mut cpu = Cpu6502::new();
        let implied_rom = [
            0xA0, 0x0a, // LDY #$0a
            0x98, // TYA
            0xA0, 0x00, // LDY #$00
            0x98, // TYA
            0xA0, 0xff, // LDY #$ff
            0x98, // TYA
        ];
        cpu.load_rom_from_memory(&implied_rom);

        cpu.step(); // LDY #$0a
        cpu.step(); // TYA
        assert_eq!(cpu.y, cpu.a);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));

        cpu.step(); // LDY #$00
        cpu.step(); // TYA
        assert_eq!(cpu.y, cpu.a);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(true, cpu.get_flag(Flag::Zero));

        cpu.step(); // LDY #$ff
        cpu.step(); // TYA
        assert_eq!(cpu.y, cpu.a);
        assert_eq!(true, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));
    }

    #[test]
    fn tsx_test() {
        let mut cpu = Cpu6502::new();

        // WARN: needs more test
        let implied_rom = [
            0xBA, // TSX
        ];
        cpu.load_rom_from_memory(&implied_rom);

        cpu.step(); // TSX {x = #$FD}
        assert_eq!(cpu.sp, cpu.x);
        assert_eq!(true, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));
    }

    #[test]
    fn txs_test() {
        let mut cpu = Cpu6502::new();
        let implied_rom = [
            0xA2, 0x0a, // LDX #$0a
            0x9a, // TXS
            0xA2, 0x00, // LDX #$00
            0x9a, // TXS
            0xA2, 0xff, // LDX #$ff
            0x9a, // TXS
        ];

        cpu.load_rom_from_memory(&implied_rom);

        cpu.step(); // LDX #$0a
        cpu.step(); // TXS
        assert_eq!(cpu.x, cpu.sp);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));

        cpu.step(); // LDX #$00
        cpu.step(); // TXS
        assert_eq!(cpu.x, cpu.sp);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(true, cpu.get_flag(Flag::Zero));

        cpu.step(); // LDX #$ff
        cpu.step(); // TXS
        assert_eq!(cpu.x, cpu.sp);
        assert_eq!(true, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));
    }

    #[test]
    fn pha_test() {
        let mut cpu = Cpu6502::new();
        let implied_rom = [
            0xA9u8, 0xaa, // LDA #$aa
            0x48, // PHA
            0xA9u8, 0xff, // LDA #$ff
            0x48, // PHA
        ];
        cpu.load_rom_from_memory(&implied_rom);

        cpu.step(); // LDA #$aa
        cpu.step(); // PHA
        assert_eq!(cpu.sp, 0xFE);
        assert_eq!(cpu.peek_stack(), 0xaa);

        cpu.step(); // LDA #$ff
        cpu.step(); // PHA
        assert_eq!(cpu.sp, 0xFD);
        assert_eq!(cpu.peek_stack(), 0xff);
    }

    #[test]
    fn pla_test() {
        let mut cpu = Cpu6502::new();
        let implied_rom = [
            0xA9u8, 0x0a, // LDA #$0a
            0x48, // PHA
            0xA9, 0xff, // LDA #$ff
            0x48, // PHA
            0xA9, 0x00, // LDA #$00
            0x48, // PHA
            0x68, // PLA {A=#$00, Z=1, N=0}
            0x68, // PLA {A=#$ff, Z=0, N=1}
            0x68, // PLA {A=#$0a, Z=0, N=0}
        ];
        cpu.load_rom_from_memory(&implied_rom);

        cpu.step(); // LDA #$0a
        cpu.step(); // PHA
        cpu.step(); // LDA #$ff
        cpu.step(); // PHA
        cpu.step(); // LDA #$00
        cpu.step(); // PHA

        // SP at 0xFC now
        assert_eq!(cpu.sp, 0xFC);

        cpu.step(); // PLA {A=#$00, Z=1, N=0}
        assert_eq!(cpu.sp, 0xFD);
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);

        cpu.step(); // PLA {A=#$ff, Z=0, N=1}
        assert_eq!(cpu.sp, 0xFE);
        assert_eq!(cpu.a, 0xff);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), true);

        cpu.step(); // PLA {A=#$0a, Z=0, N=0}
        assert_eq!(cpu.sp, 0xFF);
        assert_eq!(cpu.a, 0x0a);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn php_test() {
        let mut cpu = Cpu6502::new();
        let implied_rom = [
            0x8u8, // PHP
            0xA9, 0x00,  // LDA #$00
            0x8u8, // PHP
            0xA9, 0xff,  // LDA #$ff
            0x8u8, // PHP
        ];
        cpu.load_rom_from_memory(&implied_rom);

        // check
        //   1. sp
        //   2. peek stack

        // Intial sp
        assert_eq!(cpu.sp, 0xFF);

        cpu.step(); // PHP
        assert_eq!(cpu.sp, 0xFE);
        assert_eq!(cpu.peek_stack(), (Flag::Unused as u8) | 0);

        cpu.step(); // LDA #$00
        cpu.step(); // PHP
        assert_eq!(cpu.sp, 0xFD);
        assert_eq!(
            cpu.peek_stack(),
            (Flag::Zero as u8) | (Flag::Unused as u8) | 0
        );

        cpu.step(); // LDA #$ff
        cpu.step(); // PHP
        assert_eq!(cpu.sp, 0xFC);
        assert_eq!(
            cpu.peek_stack(),
            (Flag::Negative as u8) | (Flag::Unused as u8) | 0
        );
    }

    #[test]
    fn plp() {
        let mut cpu = Cpu6502::new();
        let implied_rom = [];
        cpu.load_rom_from_memory(&implied_rom);
    }
}

fn main() {
    let mut cpu = Cpu6502::new();
    let rom = [
        0xa9, 0x05, // LDA #5
        0xa9, 0x00, // LDA #0
    ];
    cpu.load_rom_from_memory(&rom);

    cpu.step(); // to stop all those fucking warnings.
    cpu.hexdump(START_PC_ADDRESS, 0x00ff);

    // test byte construction
    let low = 0b11111111u16;
    let high = 0b10001001u16;
    let combined = (high << 8) | low;

    println!("low : {:#018b}", low);
    println!("high: {:#018b}", high);
    println!("res : {:#018b}", combined);

    // stack address forming
    let sp = 0x0100u16 | (cpu.sp as u16);
    println!("sp: ${:04X}", sp);
}
