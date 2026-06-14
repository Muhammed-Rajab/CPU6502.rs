#[cfg(test)]
mod tests {
    use super::super::Cpu6502;

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
}
