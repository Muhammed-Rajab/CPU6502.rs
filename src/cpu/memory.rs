//-----------------------------------------------
// MEMORY                                       |
//-----------------------------------------------

use super::Cpu6502;

impl Cpu6502 {
    // PUBLIC (SUPER)
    pub(super) fn push_to_stack(&mut self, value: u8) {
        let addr = 0x0100u16 | (self.sp as u16);
        self.write(addr, value);
        self.sp -= 1;
    }

    pub(super) fn pull_from_stack(&mut self) -> u8 {
        self.sp += 1;
        let addr = 0x0100u16 | (self.sp as u16);
        self.read(addr)
    }

    pub(super) fn fetch_byte(&mut self) -> u8 {
        let byte = self.read(self.pc);
        self.pc += 1;
        byte
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
