//-----------------------------------------------
// INSTRUCTIONS                                 |
//-----------------------------------------------

use super::Cpu6502;
use super::flags::Flag;

impl Cpu6502 {
    /*
     * Loads a byte of memory into accumulator.
     * Sets the N and Z flags as appropriate.
     */
    pub(super) fn lda(&mut self, value: u8) {
        self.a = value;
        self.update_zn(self.a);
    }

    /*
     * Loads a byte of memory into X.
     * Sets the N and Z flags as appropriate.
     */
    pub(super) fn ldx(&mut self, value: u8) {
        self.x = value;
        self.update_zn(self.x);
    }

    /*
     * Loads a byte of memory into Y.
     * Sets the N and Z flags as appropriate.
     */
    pub(super) fn ldy(&mut self, value: u8) {
        self.y = value;
        self.update_zn(self.y);
    }

    /*
     * Stores the contents of A to memory[addr]
     * No flags affected.
     */
    pub(super) fn sta(&mut self, addr: u16) {
        self.memory[addr as usize] = self.a;
    }

    /*
     * Stores the contents of X to memory[addr]
     * No flags affected.
     */
    pub(super) fn stx(&mut self, addr: u16) {
        self.memory[addr as usize] = self.x;
    }

    /*
     * Stores the contents of Y to memory[addr]
     * No flags affected.
     */
    pub(super) fn sty(&mut self, addr: u16) {
        self.memory[addr as usize] = self.y;
    }

    /*
     * Copies the contents of A to X
     * Sets Z and N flag appropriately.
     */
    pub(super) fn tax(&mut self) {
        self.x = self.a;
        self.update_zn(self.x);
    }

    /*
     * Copies the contents of A to Y
     * Sets Z and N flag appropriately.
     */
    pub(super) fn tay(&mut self) {
        self.y = self.a;
        self.update_zn(self.y);
    }

    /*
     * Copies the contents of X to A
     * Sets Z and N flag appropriately.
     */
    pub(super) fn txa(&mut self) {
        self.a = self.x;
        self.update_zn(self.a);
    }

    /*
     * Copies the contents of Y to A
     * Sets Z and N flag appropriately.
     */
    pub(super) fn tya(&mut self) {
        self.a = self.y;
        self.update_zn(self.a);
    }

    /*
     * Copies the contents of SP to X
     * Sets Z and N flag appropriately.
     */
    pub(super) fn tsx(&mut self) {
        self.x = self.sp;
        self.update_zn(self.x);
    }

    /*
     * Copies the contents of X to SP
     * No flags affected.
     */
    pub(super) fn txs(&mut self) {
        self.sp = self.x;
    }

    /*
     * Pushes a copy of A on to the stack
     * No flags affected.
     * SP decreased
     */
    pub(super) fn pha(&mut self) {
        self.push_to_stack(self.a);
    }

    /*
     * Pulls 8-bit value from stack and into A.
     * Sets Z and N flags appropriately.
     * SP increases
     */
    pub(super) fn pla(&mut self) {
        self.a = self.pull_from_stack();
        self.update_zn(self.a);
    }

    /*
     * Pushes a copy of processor status on to the stack
     * No flags affected.
     * SP decreased
     */
    pub(super) fn php(&mut self) {
        self.push_to_stack(self.status);
    }

    /*
     * Pulls 8-bit value from stack and into processor status flags.
     * The flag will take new state determined by the value.
     * SP increases
     */
    pub(super) fn plp(&mut self) {
        self.status = self.pull_from_stack();
    }

    /*
     * Adds one to value held at a specific memory location.
     * Sets Z and N flags appropriately.
     */
    pub(super) fn inc(&mut self, addr: u16) {
        let val = self.read(addr).wrapping_add(1);
        self.write(addr, val);
        self.update_zn(val);
    }

    /*
     * Subtracts one from value held at a specific memory location.
     * Sets Z and N flags appropriately.
     */
    pub(super) fn dec(&mut self, addr: u16) {
        let val = self.read(addr).wrapping_sub(1);
        self.write(addr, val);
        self.update_zn(val);
    }

    /*
     * Adds one to X register
     * Sets Z and N flags appropriately.
     */
    pub(super) fn inx(&mut self) {
        self.x = self.x.wrapping_add(1);
        self.update_zn(self.x);
    }

    /*
     * Adds one to Y register
     * Sets Z and N flags appropriately.
     */
    pub(super) fn iny(&mut self) {
        self.y = self.y.wrapping_add(1);
        self.update_zn(self.y);
    }

    /*
     * Subs one from X register
     * Sets Z and N flags appropriately.
     */
    pub(super) fn dex(&mut self) {
        self.x = self.x.wrapping_sub(1);
        self.update_zn(self.x);
    }

    /*
     * Subs one from Y register
     * Sets Z and N flags appropriately.
     */
    pub(super) fn dey(&mut self) {
        self.y = self.y.wrapping_sub(1);
        self.update_zn(self.y);
    }

    /*
     * Set the carry flag to zero.
     */
    pub(super) fn clc(&mut self) {
        self.set_flag(super::flags::Flag::Carry, false);
    }

    /*
     * Set the carry flag to one.
     */
    pub(super) fn sec(&mut self) {
        self.set_flag(super::flags::Flag::Carry, true);
    }

    /*
     * Adds value to Accumulator.
     * Sets Z, N, and C flags appropriately.
     *
     * A = A + val + C
     *
     *Overflow if:
     *  (A and M have same sign)
     *  AND
     *  (A and Result have different sign)
     */
    pub(super) fn adc(&mut self, val: u8) {
        let carry_in = if self.get_flag(Flag::Carry) { 1 } else { 0 };
        let sum = self.a as u16 + val as u16 + carry_in as u16;
        let result = sum as u8;

        // carry check
        let carry = sum > 0xFF;

        // overflow check
        let a_neg = self.a & 0x80;
        let val_neg = val & 0x80;
        let res_neg = result & 0x80;
        let overflow = (a_neg == val_neg) && (a_neg != res_neg);

        // set a
        self.a = result;

        self.update_czvn(result, carry, overflow);
    }

    /*
     * A = A - val - (1 - Carry)
     * Sets Z, N, and C flags appropriately.
     *
     * NOTE: check out the math behind it. it's fascinating. explains a lot why
     * we only had a 16-bit adder in NAND2Tetris
     *
     *Overflow if:
     *  (A and M have different signs)
     *  AND
     *  (A and Result have different sign)
     */
    pub(super) fn sbc(&mut self, val: u8) {
        self.adc(!val);
    }

    /*
     * A logical AND is performed bit by bit on A.
     * Sets Z and N appropriately.
     */
    pub(super) fn and(&mut self, val: u8) {
        self.a = self.a & val;
        self.update_zn(self.a);
    }

    /*
     * A logical inclusive OR is performed bit by bit on A.
     * Sets Z and N appropriately.
     */
    pub(super) fn ora(&mut self, val: u8) {
        self.a = self.a | val;
        self.update_zn(self.a);
    }

    /*
     * An exclusive OR is performed bit by bit on A.
     * Sets Z and N appropriately.
     */
    pub(super) fn eor(&mut self, val: u8) {
        self.a = self.a ^ val;
        self.update_zn(self.a);
    }

    /*
     * Compares A with val.
     * Sets Z, C, and N flags appropriately.
     *
     * Operation performed: A - M
     *
     * C -> set if A >= M
     * Z -> set if A = M
     * N -> set if bit 7 of the result is set
     */
    pub(super) fn cmp(&mut self, val: u8) {
        let result = self.a.wrapping_sub(val);

        self.set_flag(Flag::Carry, self.a >= val);
        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(Flag::Negative, (result & 0x80) != 0);
    }

    /*
     * Compares X with val.
     * Sets Z, C, and N flags appropriately.
     *
     * Operation performed: X - M
     *
     * C -> set if X >= M
     * Z -> set if X = M
     * N -> set if bit 7 of the result is set
     */
    pub(super) fn cpx(&mut self, val: u8) {
        let result = self.x.wrapping_sub(val);

        self.set_flag(Flag::Carry, self.x >= val);
        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(Flag::Negative, (result & 0x80) != 0);
    }

    /*
     * Compares Y with val.
     * Sets Z, C, and N flags appropriately.
     *
     * Operation performed: Y - M
     *
     * C -> set if Y >= M
     * Z -> set if Y = M
     * N -> set if bit 7 of the result is set
     */
    pub(super) fn cpy(&mut self, val: u8) {
        let result = self.y.wrapping_sub(val);

        self.set_flag(Flag::Carry, self.y >= val);
        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(Flag::Negative, (result & 0x80) != 0);
    }

    /*
     * Adds relative displacement to the PC
     * if Z flag is set, causing a branch to a new location.
     *
     * BEQ +5
     *
     * BEQ -3
     */
    pub(super) fn beq(&mut self, offset: i8) {
        if self.get_flag(Flag::Zero) {
            // i16 -> sign extension
            self.pc = self.pc.wrapping_add_signed(offset as i16);
        }
    }

    /*
     * Adds relative displacement to the PC
     * if Z flag is clear, causing a branch to a new location.
     *
     * BNE +5
     *
     * BNE -3
     */
    pub(super) fn bne(&mut self, offset: i8) {
        if !self.get_flag(Flag::Zero) {
            // i16 -> signed extension
            self.pc = self.pc.wrapping_add_signed(offset as i16);
        }
    }

    /*
     * Adds relative displacement to the PC
     * if C flag is clear, causing a branch to new location.
     *
     * BCC +5
     *
     * BCC -3
     */
    pub(super) fn bcc(&mut self, offset: i8) {
        if !self.get_flag(Flag::Carry) {
            self.pc = self.pc.wrapping_add_signed(offset as i16);
        }
    }

    /*
     * Adds relative displacement to the PC
     * if C flag is set, causing a branch to new location.
     *
     * BCS +5
     *
     * BCS -3
     */
    pub(super) fn bcs(&mut self, offset: i8) {
        if self.get_flag(Flag::Carry) {
            self.pc = self.pc.wrapping_add_signed(offset as i16);
        }
    }

    /*
     * Adds relative displacement to the PC
     * if N flag is set, causing a branch to new location.
     *
     * BMI +5
     *
     * BMI -3
     */
    pub(super) fn bmi(&mut self, offset: i8) {
        if self.get_flag(Flag::Negative) {
            self.pc = self.pc.wrapping_add_signed(offset as i16);
        }
    }

    /*
     * Adds relative displacement to the PC
     * if N flag is clear, causing a branch to new location.
     *
     * BPL +5
     *
     * BPL -3
     */
    pub(super) fn bpl(&mut self, offset: i8) {
        if !self.get_flag(Flag::Negative) {
            self.pc = self.pc.wrapping_add_signed(offset as i16);
        }
    }

    /*
     * Adds relative displacement to the PC
     * if V flag is clear, causing a branch to new location.
     *
     * BVC +5
     *
     * BVC -3
     */
    pub(super) fn bvc(&mut self, offset: i8) {
        if !self.get_flag(Flag::Overflow) {
            self.pc = self.pc.wrapping_add_signed(offset as i16);
        }
    }

    /*
     * Adds relative displacement to the PC
     * if V flag is set, causing a branch to new location.
     *
     * BVS +5
     *
     * BVS -3
     */
    pub(super) fn bvs(&mut self, offset: i8) {
        if self.get_flag(Flag::Overflow) {
            self.pc = self.pc.wrapping_add_signed(offset as i16);
        }
    }
}
