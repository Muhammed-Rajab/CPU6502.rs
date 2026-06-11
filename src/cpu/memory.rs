//-----------------------------------------------
// MEMORY                                       |
//-----------------------------------------------

use super::Cpu6502;

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
