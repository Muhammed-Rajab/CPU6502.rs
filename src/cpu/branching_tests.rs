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
}
