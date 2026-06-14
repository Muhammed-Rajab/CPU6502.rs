#[cfg(test)]
mod tests {

    use super::super::super::Cpu6502;
    use super::super::super::flags::Flag;

    #[test]
    fn and_test() {
        let mut cpu = Cpu6502::new();

        let implied_rom = [
            0xA9u8, 0xFF, // LDA #$FF
            0x29, 0x0F, // AND #$0F {A=#$0F, Z=0, N=0}
            0xA9, 0x80, // LDA #$80
            0x29, 0x80, // AND #$80 {A=#$80, Z=0, N=1}
            0xA9, 0x55, // LDA #$55
            0x29, 0xAA, // AND #$AA {A=#$00, Z=1, N=0}
        ];

        cpu.load_program_from_memory(&implied_rom);

        cpu.step(); // LDA #$FF
        cpu.step(); // AND #$0F
        assert_eq!(cpu.a, 0x0F);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);

        cpu.step(); // LDA #$80
        cpu.step(); // AND #$80
        assert_eq!(cpu.a, 0x80);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), true);

        cpu.step(); // LDA #$55
        cpu.step(); // AND #$AA
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn ora_test() {
        let mut cpu = Cpu6502::new();

        let implied_rom = [
            0xA9u8, 0x00, // LDA #$00
            0x09, 0x0F, // ORA #$0F {A=#$0F, Z=0, N=0}
            0xA9, 0x00, // LDA #$00
            0x09, 0x80, // ORA #$80 {A=#$80, Z=0, N=1}
            0xA9, 0x00, // LDA #$00
            0x09, 0x00, // ORA #$00 {A=#$00, Z=1, N=0}
        ];

        cpu.load_program_from_memory(&implied_rom);

        cpu.step(); // LDA #$00
        cpu.step(); // ORA #$0F
        assert_eq!(cpu.a, 0x0F);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);

        cpu.step(); // LDA #$00
        cpu.step(); // ORA #$80
        assert_eq!(cpu.a, 0x80);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), true);

        cpu.step(); // LDA #$00
        cpu.step(); // ORA #$00
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn eor_test() {
        let mut cpu = Cpu6502::new();

        let implied_rom = [
            0xA9u8, 0xFF, // LDA #$FF
            0x49, 0xFF, // EOR #$FF {A=#$00, Z=1, N=0}
            0xA9, 0x00, // LDA #$00
            0x49, 0x80, // EOR #$80 {A=#$80, Z=0, N=1}
            0xA9, 0x55, // LDA #$55
            0x49, 0x0F, // EOR #$0F {A=#$5A, Z=0, N=0}
        ];

        cpu.load_program_from_memory(&implied_rom);

        cpu.step(); // LDA #$FF
        cpu.step(); // EOR #$FF
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);

        cpu.step(); // LDA #$00
        cpu.step(); // EOR #$80
        assert_eq!(cpu.a, 0x80);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), true);

        cpu.step(); // LDA #$55
        cpu.step(); // EOR #$0F
        assert_eq!(cpu.a, 0x5A);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn bit_sets_zero_overflow_and_negative_flags() {
        let mut cpu = Cpu6502::new();

        cpu.a = 0x0F;

        cpu.write(0x0042, 0xC0); // 1100_0000

        let program = [
            0x24, 0x42, // BIT $42
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert!(cpu.get_flag(Flag::Zero));
        assert!(cpu.get_flag(Flag::Overflow));
        assert!(cpu.get_flag(Flag::Negative));

        assert_eq!(cpu.a, 0x0F);
    }

    #[test]
    fn asl_accumulator_sets_carry_and_zero() {
        let mut cpu = Cpu6502::new();

        cpu.a = 0x80;

        cpu.asl_accumulator();

        assert_eq!(cpu.a, 0x00);
        assert!(cpu.get_flag(Flag::Carry));
        assert!(cpu.get_flag(Flag::Zero));
        assert!(!cpu.get_flag(Flag::Negative));
    }

    #[test]
    fn asl_memory_sets_carry_and_zero() {
        let mut cpu = Cpu6502::new();

        cpu.write(0x0042, 0x80);

        let program = [
            0x06, 0x42, // ASL $42
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.read(0x0042), 0x00);
        assert!(cpu.get_flag(Flag::Carry));
        assert!(cpu.get_flag(Flag::Zero));
        assert!(!cpu.get_flag(Flag::Negative));
    }
}
