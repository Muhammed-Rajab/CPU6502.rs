#[cfg(test)]
mod tests {

    use super::super::super::Cpu6502;

    #[test]
    fn sta_test() {
        // Absolute Mode
        let mut cpu = Cpu6502::new();
        let absolute_rom = [
            0xA9, 0x42, //  LDA #$42
            0x8Du8, 0x34, 0x12, // STA $1234
        ];

        cpu.load_program_from_memory(&absolute_rom);

        // 1st STA
        cpu.step();
        cpu.step();
        assert_eq!(cpu.read(0x1234), 0x42);

        // Zero Page Mode
        let mut cpu = Cpu6502::new();
        let zero_page_rom = [
            0xA9, 0x42, //  LDA #$42
            0x85u8, 0xff, // STA $ff
        ];

        cpu.load_program_from_memory(&zero_page_rom);

        // 1st STA
        cpu.step();
        cpu.step();
        assert_eq!(cpu.read(0xff), 0x42);
    }

    #[test]
    fn stx_test() {
        // Absolute Mode
        let mut cpu = Cpu6502::new();
        let absolute_rom = [
            0xA2, 0x42, //  LDX #$42
            0x8Eu8, 0x34, 0x12, // STX $1234
        ];

        cpu.load_program_from_memory(&absolute_rom);

        // 1st STX
        cpu.step();
        cpu.step();
        assert_eq!(cpu.read(0x1234), 0x42);

        // Zero Page Mode
        let mut cpu = Cpu6502::new();
        let zero_page_rom = [
            0xA2, 0x42, //  LDX #$42
            0x86u8, 0xff, // STX $ff
        ];

        cpu.load_program_from_memory(&zero_page_rom);

        // 1st STX
        cpu.step();
        cpu.step();
        assert_eq!(cpu.read(0xff), 0x42);
    }

    #[test]
    fn sty_test() {
        // Absolute Mode
        let mut cpu = Cpu6502::new();
        let absolute_rom = [
            0xA0, 0x42, //  LDY #$42
            0x8Cu8, 0x34, 0x12, // STY $1234
        ];

        cpu.load_program_from_memory(&absolute_rom);

        // 1st STY
        cpu.step();
        cpu.step();
        assert_eq!(cpu.read(0x1234), 0x42);

        // Zero Page Mode
        let mut cpu = Cpu6502::new();
        let zero_page_rom = [
            0xA0, 0x42, //  LDY #$42
            0x84u8, 0xff, // STY $ff
        ];

        cpu.load_program_from_memory(&zero_page_rom);

        // 1st STX
        cpu.step();
        cpu.step();
        assert_eq!(cpu.read(0xff), 0x42);
    }
}
