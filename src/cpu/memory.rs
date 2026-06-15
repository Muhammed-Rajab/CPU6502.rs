//-----------------------------------------------
// MEMORY                                       |
//-----------------------------------------------

use super::Cpu6502;
use super::addressing_modes::AddressingMode;

impl Cpu6502 {
    // PUBLIC (SUPER)

    pub(super) fn fetch_irq_brk_vector(&self) -> u16 {
        let low = self.read(0xFFFE) as u16;
        let high = self.read(0xFFFF) as u16;
        let vector = (high << 8) | low;
        vector
    }

    pub(super) fn push_to_stack(&mut self, value: u8) {
        let addr = 0x0100u16 | (self.sp as u16);
        self.write(addr, value);
        self.sp = self.sp.wrapping_sub(1);
    }

    pub(super) fn push_word_to_stack(&mut self, word: u16) {
        let low = (word & 0x00FF) as u8;
        let high = ((word & 0xFF00) >> 8) as u8;
        self.push_to_stack(high);
        self.push_to_stack(low);
    }

    pub(super) fn pull_from_stack(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        let addr = 0x0100u16 | (self.sp as u16);
        self.read(addr)
    }

    pub(super) fn pull_word_from_stack(&mut self) -> u16 {
        let low = self.pull_from_stack() as u16;
        let high = self.pull_from_stack() as u16;
        let word = (high << 8) | low;
        word
    }

    pub(super) fn fetch_byte(&mut self) -> u8 {
        let byte = self.read(self.pc);
        self.pc += 1;
        byte
    }

    pub(super) fn fetch_word(&mut self) -> u16 {
        let low = self.fetch_byte() as u16;
        let high = self.fetch_byte() as u16;
        let addr = (high << 8) | low;
        addr
    }

    pub(super) fn fetch_value(&mut self, mode: AddressingMode) -> u8 {
        match mode {
            AddressingMode::Immediate => self.fetch_byte(),

            AddressingMode::ZeroPage => {
                let addr = self.fetch_byte() as u16;
                self.read(addr)
            }

            AddressingMode::ZeroPageX => {
                let base = self.fetch_byte();
                let addr = base.wrapping_add(self.x) as u16;
                self.read(addr)
            }

            AddressingMode::ZeroPageY => {
                let base = self.fetch_byte();
                let addr = base.wrapping_add(self.y) as u16;
                self.read(addr)
            }

            AddressingMode::Absolute => {
                let addr = self.fetch_word();
                self.read(addr)
            }

            AddressingMode::AbsoluteX => {
                let base = self.fetch_word();
                let addr = base + self.x as u16;
                self.read(addr)
            }

            AddressingMode::AbsoluteY => {
                let base = self.fetch_word();
                let addr = base + self.y as u16;
                self.read(addr)
            }

            AddressingMode::IndexedIndirect => {
                let base = self.fetch_byte();
                let zp_addr = base.wrapping_add(self.x);

                let low_addr = zp_addr;
                let high_addr = zp_addr.wrapping_add(1);

                let low = self.read(low_addr as u16) as u16;
                let high = self.read(high_addr as u16) as u16;
                let addr = (high << 8) | low;

                self.read(addr)
            }

            AddressingMode::IndirectIndexed => {
                let base = self.fetch_byte();

                let low = self.read(base as u16) as u16;
                let high = self.read(base.wrapping_add(1) as u16) as u16;

                let pointer = (high << 8) | low;

                let addr = pointer.wrapping_add(self.y as u16);

                self.read(addr)
            }

            AddressingMode::Relative => self.fetch_byte(),

            _ => panic!("invalid addressingmode in fetch_value"),
        }
    }

    pub(super) fn fetch_addr(&mut self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::ZeroPage => self.fetch_byte() as u16,

            AddressingMode::ZeroPageX => {
                let base = self.fetch_byte();
                let addr = base.wrapping_add(self.x) as u16;
                addr
            }

            AddressingMode::ZeroPageY => {
                let base = self.fetch_byte();
                let addr = base.wrapping_add(self.y) as u16;
                addr
            }

            AddressingMode::Absolute => self.fetch_word(),

            AddressingMode::AbsoluteX => {
                let base = self.fetch_word();
                let addr = base + self.x as u16;
                addr
            }

            AddressingMode::AbsoluteY => {
                let base = self.fetch_word();
                let addr = base + self.y as u16;
                addr
            }

            /*
             * Only used by JMP. Essentially simulates the bug in hardware.
             */
            AddressingMode::Indirect => {
                let pointer = self.fetch_word();

                let ends_with_ff = (pointer & 0x00FF) == 0xFF;

                let low = self.read(pointer) as u16;
                let high = (if ends_with_ff {
                    self.read(pointer & 0xFF00)
                } else {
                    self.read(pointer.wrapping_add(1))
                }) as u16;
                let addr = (high << 8) | low;

                addr
            }

            AddressingMode::IndexedIndirect => {
                let base = self.fetch_byte();
                let zp_addr = base.wrapping_add(self.x);

                let low_addr = zp_addr;
                let high_addr = zp_addr.wrapping_add(1);

                let low = self.read(low_addr as u16) as u16;
                let high = self.read(high_addr as u16) as u16;
                let addr = (high << 8) | low;

                addr
            }

            AddressingMode::IndirectIndexed => {
                let base = self.fetch_byte();

                let low = self.read(base as u16) as u16;
                let high = self.read(base.wrapping_add(1) as u16) as u16;

                let pointer = (high << 8) | low;

                let addr = pointer.wrapping_add(self.y as u16);

                addr
            }

            _ => panic!("invalid addressingmode in fetch_addr"),
        }
    }

    // PUBLIC
    pub fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    pub fn peek_stack(&mut self) -> u8 {
        let top_sp = self.sp.wrapping_add(1);
        let addr = 0x0100u16 | top_sp as u16;
        self.read(addr)
    }

    pub fn load_program_from_memory(&mut self, rom: &[u8]) {
        let start = super::START_PC_ADDRESS as usize;
        let end = start + rom.len();

        if end > 65536 {
            panic!("rom too big");
        }

        self.memory[start..end].copy_from_slice(rom);
    }
}
