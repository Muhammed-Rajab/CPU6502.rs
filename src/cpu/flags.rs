//-----------------------------------------------
// FLAGS                                        |
//-----------------------------------------------

use super::Cpu6502;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Flag {
    Carry = 1 << 0,
    Zero = 1 << 1,
    Interrupt = 1 << 2,
    Decimal = 1 << 3,
    Break = 1 << 4,
    Unused = 1 << 5,
    Overflow = 1 << 6,
    Negative = 1 << 7,
}

impl Cpu6502 {
    // PUBLIC SUPER
    pub(super) fn set_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.status |= flag as u8;
        } else {
            self.status &= !(flag as u8);
        }

        // Always set
        self.status |= Flag::Unused as u8;
    }

    pub(super) fn update_zn(&mut self, value: u8) {
        self.set_flag(Flag::Zero, value == 0);
        self.set_flag(Flag::Negative, (value & 0x80) != 0);
    }

    pub(super) fn update_czvn(&mut self, value: u8, overflow: bool) {
        self.set_flag(Flag::Carry, overflow);
        self.set_flag(Flag::Zero, value == 0);
        self.set_flag(Flag::Negative, (value & 0x80) != 0);
    }

    // PUBLIC
    pub fn get_flag(&self, flag: Flag) -> bool {
        (self.status & (flag as u8)) != 0
    }
}
