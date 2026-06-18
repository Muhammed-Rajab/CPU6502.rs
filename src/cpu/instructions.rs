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
        // on real NMOS 6502, PHP pushes a modified copy of the status register
        // where    bit 4(B, break) = 1
        //          bit 5 (unused) = 1
        self.push_to_stack(self.status | 0x30);
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
     * Clears the carry flag.
     */
    pub(super) fn clc(&mut self) {
        self.set_flag(super::flags::Flag::Carry, false);
    }

    /*
     * Sets the carry flag.
     */
    pub(super) fn sec(&mut self) {
        self.set_flag(super::flags::Flag::Carry, true);
    }

    /*
     * Clears the overflow flag.
     */
    pub(super) fn clv(&mut self) {
        self.set_flag(super::flags::Flag::Overflow, false);
    }

    /*
     * Clears the interrupt disable flag.
     */
    pub(super) fn cli(&mut self) {
        self.set_flag(super::flags::Flag::Interrupt, false);
    }

    /*
     * Sets the interrupt disable flag.
     */
    pub(super) fn sei(&mut self) {
        self.set_flag(super::flags::Flag::Interrupt, true);
    }

    /*
     * Clears the decimal mode flag.
     */
    pub(super) fn cld(&mut self) {
        self.set_flag(super::flags::Flag::Decimal, false);
    }

    /*
     * Sets the decimal mode flag.
     */
    pub(super) fn sed(&mut self) {
        self.set_flag(super::flags::Flag::Decimal, true);
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
    pub(super) fn adc_binary(&mut self, val: u8) {
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

    pub(super) fn adc_bcd(&mut self, val: u8) {
        let carry_in = if self.get_flag(Flag::Carry) { 1 } else { 0 };

        // binary addition
        let binary_sum = self.a as u16 + val as u16 + carry_in as u16;
        let binary_result = binary_sum as u8;

        // overflow
        let a_neg = self.a & 0x80;
        let val_neg = val & 0x80;
        let res_neg = binary_result & 0x80;

        let overflow = (a_neg == val_neg) && (a_neg != res_neg);

        // flag setting
        self.set_flag(Flag::Zero, binary_result == 0);
        self.set_flag(Flag::Negative, res_neg != 0);
        self.set_flag(Flag::Overflow, overflow);

        // bcd
        let mut result = binary_sum;

        if (result & 0x0F) > 9 {
            result += 0x06;
        }

        if result > 0x99 {
            result += 0x60;
        }

        self.set_flag(Flag::Carry, result > 0x99);

        self.a = result as u8;
    }

    pub(super) fn adc(&mut self, val: u8) {
        if self.get_flag(Flag::Decimal) {
            self.adc_bcd(val);
        } else {
            self.adc_binary(val);
        }
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
     * Tests if one or more bits are set in a target memory location.
     *
     * Z = A & M == 0
     * V = bit 6 of M
     * N = bit 7 of M
     * */
    pub(super) fn bit(&mut self, val: u8) {
        let z = (self.a & val) == 0;
        let v = (val & 0x40) != 0;
        let n = (val & 0x80) != 0;

        self.set_flag(Flag::Zero, z);
        self.set_flag(Flag::Overflow, v);
        self.set_flag(Flag::Negative, n);
    }

    pub(super) fn asl_accumulator(&mut self) {
        let c = (self.a & 0x80) != 0;
        self.a = self.a.wrapping_shl(1);

        self.set_flag(Flag::Carry, c);
        self.update_zn(self.a);
    }

    pub(super) fn asl_memory(&mut self, addr: u16) {
        let mut val = self.read(addr);

        let c = (val & 0x80) != 0;

        val = val.wrapping_shl(1);
        self.write(addr, val);

        self.set_flag(Flag::Carry, c);
        self.update_zn(val);
    }

    pub(super) fn lsr_accumulator(&mut self) {
        let c = (self.a & 0x01) != 0;
        self.a = self.a >> 1;

        self.set_flag(Flag::Carry, c);
        self.update_zn(self.a);
    }

    pub(super) fn lsr_memory(&mut self, addr: u16) {
        let mut val = self.read(addr);

        let c = (val & 0x01) != 0;

        val = val >> 1;
        self.write(addr, val);

        self.set_flag(Flag::Carry, c);
        self.update_zn(val);
    }

    pub(super) fn rol_accumulator(&mut self) {
        let old_c = self.get_flag(Flag::Carry);
        let old_bit_7 = self.a & 0x80;

        self.a = self.a.wrapping_shl(1);

        if old_c {
            self.a = self.a | 0x01;
        }

        self.set_flag(Flag::Carry, old_bit_7 != 0);
        self.update_zn(self.a);
    }

    pub(super) fn rol_memory(&mut self, addr: u16) {
        let mut val = self.read(addr);

        let old_c = self.get_flag(Flag::Carry);
        let old_bit_7 = val & 0x80;

        val = val.wrapping_shl(1);

        if old_c {
            val = val | 0x01;
        }

        self.write(addr, val);

        self.set_flag(Flag::Carry, old_bit_7 != 0);
        self.update_zn(val);
    }

    pub(super) fn ror_accumulator(&mut self) {
        let old_c = self.get_flag(Flag::Carry);
        let old_bit_0 = self.a & 0x01;

        self.a = self.a >> 1;

        if old_c {
            self.a = self.a | 0x80;
        }

        self.set_flag(Flag::Carry, old_bit_0 != 0);
        self.update_zn(self.a);
    }

    pub(super) fn ror_memory(&mut self, addr: u16) {
        let mut val = self.read(addr);

        let old_c = self.get_flag(Flag::Carry);
        let old_bit_0 = val & 0x01;

        val = val >> 1;

        if old_c {
            val = val | 0x80;
        }

        self.write(addr, val);

        self.set_flag(Flag::Carry, old_bit_0 != 0);
        self.update_zn(val);
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

    pub(super) fn jmp(&mut self, addr: u16) {
        self.pc = addr;
    }

    pub(super) fn jsr(&mut self, addr: u16) {
        // println!("JSR PC = {:04X}", self.pc);

        let last_byte_addr = self.pc.wrapping_sub(1);

        let low = (last_byte_addr & 0x00FF) as u8;
        let high = ((last_byte_addr & 0xFF00) >> 8) as u8;

        self.push_to_stack(high);
        self.push_to_stack(low);

        self.pc = addr;
    }

    pub(super) fn rts(&mut self) {
        let low = self.pull_from_stack() as u16;
        let high = self.pull_from_stack() as u16;

        let new_pc = ((high << 8) | low).wrapping_add(1);

        self.pc = new_pc;
    }

    pub(super) fn brk(&mut self) {
        // push pc
        self.push_word_to_stack(self.pc);

        // push status register with break flag on
        let mut status = self.status;
        status = status | 0x30;

        self.push_to_stack(status);

        // set interrupt disable flag (I)
        self.set_flag(Flag::Interrupt, true);

        // Load new PC from IRQ/BRK vector $FFFE/$FFFF
        let vector = self.fetch_irq_brk_vector();

        self.pc = vector;
    }

    pub(super) fn rti(&mut self) {
        self.status = self.pull_from_stack();
        self.pc = self.pull_word_from_stack();
    }
}
