#[cfg(test)]
mod tests {

    use super::super::super::Cpu6502;
    use super::super::super::flags::Flag;

    #[test]
    fn beq_test() {
        let mut cpu = Cpu6502::new();

        let implied_rom = [
            0xA9u8, 0x00, // LDA #$00
            0xF0, 0x02, // BEQ +2
            0xA9, 0xFF, // LDA #$FF (skipped)
            0xA9, 0x42, // LDA #$42
            0xA9, 0x01, // LDA #$01
            0xF0, 0x02, // BEQ +2 (not taken)
            0xA9, 0x69, // LDA #$69
        ];

        cpu.load_program_from_memory(&implied_rom);

        cpu.step(); // LDA #$00
        cpu.step(); // BEQ +2
        cpu.step(); // LDA #$42
        assert_eq!(cpu.a, 0x42);

        cpu.step(); // LDA #$01
        cpu.step(); // BEQ +2
        cpu.step(); // LDA #$69
        assert_eq!(cpu.a, 0x69);
    }

    #[test]
    fn bne_test() {
        let mut cpu = Cpu6502::new();

        let implied_rom = [
            0xA9u8, 0x01, // LDA #$01
            0xD0, 0x02, // BNE +2
            0xA9, 0xFF, // LDA #$FF (skipped)
            0xA9, 0x42, // LDA #$42
            0xA9, 0x00, // LDA #$00
            0xD0, 0x02, // BNE +2 (not taken)
            0xA9, 0x69, // LDA #$69
        ];

        cpu.load_program_from_memory(&implied_rom);

        cpu.step(); // LDA #$01
        cpu.step(); // BNE +2
        cpu.step(); // LDA #$42
        assert_eq!(cpu.a, 0x42);

        cpu.step(); // LDA #$00
        cpu.step(); // BNE +2
        cpu.step(); // LDA #$69
        assert_eq!(cpu.a, 0x69);
    }

    #[test]
    fn bcc_branches_only_when_carry_clear() {
        // ------------------------
        // Carry clear -> branch taken
        // ------------------------
        let mut cpu = Cpu6502::new();

        let program = [
            0x18, // CLC
            0x90, 0x02, // BCC +2
            0xA9, 0x11, // LDA #$11 (should be skipped)
            0xA9, 0x22, // LDA #$22
        ];

        cpu.load_program_from_memory(&program);

        cpu.step(); // CLC
        cpu.step(); // BCC
        cpu.step(); // LDA #$22

        assert_eq!(cpu.a, 0x22);

        // ------------------------
        // Carry set -> branch not taken
        // ------------------------
        let mut cpu = Cpu6502::new();

        let program = [
            0x38, // SEC
            0x90, 0x02, // BCC +2
            0xA9, 0x11, // LDA #$11
            0xA9, 0x22, // LDA #$22
        ];

        cpu.load_program_from_memory(&program);

        cpu.step(); // SEC
        cpu.step(); // BCC (not taken)
        cpu.step(); // LDA #$11

        assert_eq!(cpu.a, 0x11);
    }

    #[test]
    fn bcs_branches_only_when_carry_set() {
        // ------------------------
        // Carry set -> branch taken
        // ------------------------
        let mut cpu = Cpu6502::new();

        let program = [
            0x38, // SEC
            0xB0, 0x02, // BCS +2
            0xA9, 0x11, // LDA #$11 (should be skipped)
            0xA9, 0x22, // LDA #$22
        ];

        cpu.load_program_from_memory(&program);

        cpu.step(); // SEC
        cpu.step(); // BCS
        cpu.step(); // LDA #$22

        assert_eq!(cpu.a, 0x22);

        // ------------------------
        // Carry clear -> branch not taken
        // ------------------------
        let mut cpu = Cpu6502::new();

        let program = [
            0x18, // CLC
            0xB0, 0x02, // BCS +2
            0xA9, 0x11, // LDA #$11
            0xA9, 0x22, // LDA #$22
        ];

        cpu.load_program_from_memory(&program);

        cpu.step(); // CLC
        cpu.step(); // BCS (not taken)
        cpu.step(); // LDA #$11

        assert_eq!(cpu.a, 0x11);
    }

    #[test]
    fn bmi_branches_only_when_negative_set() {
        // ------------------------
        // Negative set -> branch taken
        // ------------------------
        let mut cpu = Cpu6502::new();

        let program = [
            0xA9, 0x80, // LDA #$80 (sets Negative)
            0x30, 0x02, // BMI +2
            0xA9, 0x11, // LDA #$11 (should be skipped)
            0xA9, 0x22, // LDA #$22
        ];

        cpu.load_program_from_memory(&program);

        cpu.step(); // LDA #$80
        cpu.step(); // BMI
        cpu.step(); // LDA #$22

        assert_eq!(cpu.a, 0x22);

        // ------------------------
        // Negative clear -> branch not taken
        // ------------------------
        let mut cpu = Cpu6502::new();

        let program = [
            0xA9, 0x01, // LDA #$01 (clears Negative)
            0x30, 0x02, // BMI +2
            0xA9, 0x11, // LDA #$11
            0xA9, 0x22, // LDA #$22
        ];

        cpu.load_program_from_memory(&program);

        cpu.step(); // LDA #$01
        cpu.step(); // BMI (not taken)
        cpu.step(); // LDA #$11

        assert_eq!(cpu.a, 0x11);
    }

    #[test]
    fn bpl_branches_only_when_negative_clear() {
        // ------------------------
        // Negative clear -> branch taken
        // ------------------------
        let mut cpu = Cpu6502::new();

        let program = [
            0xA9, 0x01, // LDA #$01 (clears Negative)
            0x10, 0x02, // BPL +2
            0xA9, 0x11, // LDA #$11 (should be skipped)
            0xA9, 0x22, // LDA #$22
        ];

        cpu.load_program_from_memory(&program);

        cpu.step(); // LDA #$01
        cpu.step(); // BPL
        cpu.step(); // LDA #$22

        assert_eq!(cpu.a, 0x22);

        // ------------------------
        // Negative set -> branch not taken
        // ------------------------
        let mut cpu = Cpu6502::new();

        let program = [
            0xA9, 0x80, // LDA #$80 (sets Negative)
            0x10, 0x02, // BPL +2
            0xA9, 0x11, // LDA #$11
            0xA9, 0x22, // LDA #$22
        ];

        cpu.load_program_from_memory(&program);

        cpu.step(); // LDA #$80
        cpu.step(); // BPL (not taken)
        cpu.step(); // LDA #$11

        assert_eq!(cpu.a, 0x11);
    }

    #[test]
    fn bvc_branches_only_when_overflow_clear() {
        // ------------------------
        // Overflow clear -> branch taken
        // ------------------------
        let mut cpu = Cpu6502::new();

        let program = [
            0xA9, 0x01, // LDA #$01
            0x50, 0x02, // BVC +2
            0xA9, 0x11, // LDA #$11 (should be skipped)
            0xA9, 0x22, // LDA #$22
        ];

        cpu.load_program_from_memory(&program);

        cpu.step(); // LDA #$01
        cpu.step(); // BVC
        cpu.step(); // LDA #$22

        assert_eq!(cpu.a, 0x22);

        // ------------------------
        // Overflow set -> branch not taken
        // ------------------------
        let mut cpu = Cpu6502::new();

        let program = [
            0xA9, 0x7F, // LDA #$7F
            0x69, 0x01, // ADC #$01 -> 0x80, sets Overflow
            0x50, 0x02, // BVC +2
            0xA9, 0x11, // LDA #$11
            0xA9, 0x22, // LDA #$22
        ];

        cpu.load_program_from_memory(&program);

        cpu.step(); // LDA #$7F
        cpu.step(); // ADC #$01
        cpu.step(); // BVC (not taken)
        cpu.step(); // LDA #$11

        assert_eq!(cpu.a, 0x11);
    }

    #[test]
    fn bvs_branches_only_when_overflow_set() {
        // ------------------------
        // Overflow set -> branch taken
        // ------------------------
        let mut cpu = Cpu6502::new();

        let program = [
            0xA9, 0x7F, // LDA #$7F
            0x69, 0x01, // ADC #$01 -> 0x80, sets Overflow
            0x70, 0x02, // BVS +2
            0xA9, 0x11, // LDA #$11 (should be skipped)
            0xA9, 0x22, // LDA #$22
        ];

        cpu.load_program_from_memory(&program);

        cpu.step(); // LDA #$7F
        cpu.step(); // ADC #$01
        cpu.step(); // BVS
        cpu.step(); // LDA #$22

        assert_eq!(cpu.a, 0x22);

        // ------------------------
        // Overflow clear -> branch not taken
        // ------------------------
        let mut cpu = Cpu6502::new();

        let program = [
            0xA9, 0x01, // LDA #$01
            0x70, 0x02, // BVS +2
            0xA9, 0x11, // LDA #$11
            0xA9, 0x22, // LDA #$22
        ];

        cpu.load_program_from_memory(&program);

        cpu.step(); // LDA #$01
        cpu.step(); // BVS (not taken)
        cpu.step(); // LDA #$11

        assert_eq!(cpu.a, 0x11);
    }
}
