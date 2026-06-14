#[cfg(test)]
mod tests {

    use super::super::super::Cpu6502;
    use super::super::super::flags::Flag;

    #[test]
    fn lda_test() {
        let mut cpu = Cpu6502::new();

        // Immediate Mode
        let immediate_rom = [
            0xA9u8, 0x10, // LDA #$10
            0xA9, 0x00, // LDA #$00
            0xA9, 0xFF, // LDA #$FF
        ];

        cpu.load_program_from_memory(&immediate_rom);

        // 1st LDA
        cpu.step();
        assert_eq!(0x10, cpu.a);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));

        // 2nd LDA
        cpu.step();
        assert_eq!(0x00, cpu.a);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(true, cpu.get_flag(Flag::Zero));

        // 3rd LDA
        cpu.step();
        assert_eq!(0xFF, cpu.a);
        assert_eq!(true, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));
    }

    #[test]
    fn ldx_test() {
        let mut cpu = Cpu6502::new();

        // Immediate Mode
        let immediate_rom = [
            0xA2u8, 0x10, // LDX #$10
            0xA2, 0x00, // LDX #$00
            0xA2, 0xFF, // LDX #$FF
        ];

        cpu.load_program_from_memory(&immediate_rom);

        // 1st LDX
        cpu.step();
        assert_eq!(0x10, cpu.x);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));

        // 2nd LDX
        cpu.step();
        assert_eq!(0x00, cpu.x);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(true, cpu.get_flag(Flag::Zero));

        // 3rd LDX
        cpu.step();
        assert_eq!(0xFF, cpu.x);
        assert_eq!(true, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));
    }

    #[test]
    fn ldy_test() {
        let mut cpu = Cpu6502::new();

        // Immediate Mode
        let immediate_rom = [
            0xA0u8, 0x10, // LDY #$10
            0xA0, 0x00, // LDY #$00
            0xA0, 0xFF, // LDY #$FF
        ];

        cpu.load_program_from_memory(&immediate_rom);

        // 1st LDY
        cpu.step();
        assert_eq!(0x10, cpu.y);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));

        // 2nd LDY
        cpu.step();
        assert_eq!(0x00, cpu.y);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(true, cpu.get_flag(Flag::Zero));

        // 3rd LDY
        cpu.step();
        assert_eq!(0xFF, cpu.y);
        assert_eq!(true, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));
    }
}
