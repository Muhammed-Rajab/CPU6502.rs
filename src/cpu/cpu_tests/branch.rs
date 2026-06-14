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
}
