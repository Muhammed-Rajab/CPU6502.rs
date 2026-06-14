//-----------------------------------------------
// TESTS                                        |
//-----------------------------------------------

// #[cfg(test)]
// mod tests {
//
//     use super::super::super::Cpu6502;
//     use super::super::super::flags::Flag;
// }

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

    //-----------------------------------------------------------------
    // A L L  M O D E  T E S T
    //-----------------------------------------------------------------

    // LDA
    #[test]
    fn lda_all_addressing_modes_basic_behavior() {
        // ------------------------
        // Immediate
        // ------------------------
        let mut cpu = Cpu6502::new();

        let program = [
            0xA9u8, 0x00, // LDA #$00
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);

        // ------------------------
        // Zero Page
        // ------------------------
        let mut cpu = Cpu6502::new();

        cpu.write(0x0042, 0x7F);

        let program = [
            0xA5u8, 0x42, // LDA $42
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.a, 0x7F);

        // ------------------------
        // Absolute
        // ------------------------
        let mut cpu = Cpu6502::new();

        cpu.write(0x1234, 0x80);

        let program = [
            0xADu8, 0x34, 0x12, // LDA $1234
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.a, 0x80);
        assert_eq!(cpu.get_flag(Flag::Negative), true);

        // ------------------------
        // Absolute X
        // ------------------------
        let mut cpu = Cpu6502::new();

        cpu.x = 0x10;
        cpu.write(0x2000 + 0x10, 0x55);

        let program = [
            0xBDu8, 0x00, 0x20, // LDA $2000,X
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.a, 0x55);

        // ------------------------
        // Absolute Y
        // ------------------------
        let mut cpu = Cpu6502::new();

        cpu.y = 0x10;
        cpu.write(0x3000 + 0x10, 0x66);

        let program = [
            0xB9u8, 0x00, 0x30, // LDA $3000,Y
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.a, 0x66);

        // ------------------------
        // Zero Page X
        // ------------------------
        let mut cpu = Cpu6502::new();

        cpu.x = 0x05;
        cpu.write(0x0047, 0x99); // 0x42 + 0x05 = 0x47

        let program = [
            0xB5u8, 0x42, // LDA $42,X
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.a, 0x99);

        // ------------------------
        // Indexed Indirect
        // ------------------------
        let mut cpu = Cpu6502::new();

        cpu.x = 0x04;

        cpu.write(0x0024, 0x00);
        cpu.write(0x0025, 0x80);

        cpu.write(0x8000, 0x11);

        let program = [
            0xA1u8, 0x20, // LDA ($20,X)
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.a, 0x11);

        // ------------------------
        // Indirect Indexed
        // ------------------------
        let mut cpu = Cpu6502::new();

        cpu.y = 0x10;

        cpu.write(0x0030, 0x00);
        cpu.write(0x0031, 0x90);

        cpu.write(0x9000 + 0x10, 0x22);

        let program = [
            0xB1u8, 0x30, // LDA ($30),Y
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.a, 0x22);
    }

    // LDX
    #[test]
    fn ldx_all_addressing_modes_basic_behavior() {
        // ------------------------
        // Immediate
        // ------------------------
        let mut cpu = Cpu6502::new();

        let program = [
            0xA2u8, 0x00, // LDX #$00
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.x, 0x00);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);

        // ------------------------
        // Zero Page
        // ------------------------
        let mut cpu = Cpu6502::new();

        cpu.write(0x0042, 0x7F);

        let program = [
            0xA6u8, 0x42, // LDX $42
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.x, 0x7F);

        // ------------------------
        // Absolute
        // ------------------------
        let mut cpu = Cpu6502::new();

        cpu.write(0x1234, 0x80);

        let program = [
            0xAEu8, 0x34, 0x12, // LDX $1234
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.x, 0x80);
        assert_eq!(cpu.get_flag(Flag::Negative), true);

        // ------------------------
        // Absolute Y
        // ------------------------
        let mut cpu = Cpu6502::new();

        cpu.y = 0x10;
        cpu.write(0x2000 + 0x10, 0x01);

        let program = [
            0xBEu8, 0x00, 0x20, // LDX $2000,Y
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.x, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
    }

    // LDY
    #[test]
    fn ldy_all_addressing_modes_basic_behavior() {
        // ------------------------
        // Immediate
        // ------------------------
        let mut cpu = Cpu6502::new();

        let program = [
            0xA0u8, 0x00, // LDY #$00
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.y, 0x00);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);

        // ------------------------
        // Zero Page
        // ------------------------
        let mut cpu = Cpu6502::new();

        cpu.write(0x0042, 0x7F);

        let program = [
            0xA4u8, 0x42, // LDY $42
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.y, 0x7F);

        // ------------------------
        // Absolute
        // ------------------------
        let mut cpu = Cpu6502::new();

        cpu.write(0x1234, 0x80);

        let program = [
            0xACu8, 0x34, 0x12, // LDY $1234
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.y, 0x80);
        assert_eq!(cpu.get_flag(Flag::Negative), true);

        // ------------------------
        // Zero Page X
        // ------------------------
        let mut cpu = Cpu6502::new();

        cpu.x = 0x05;
        cpu.write(0x0047, 0x99); // 0x42 + 0x05 = 0x47

        let program = [
            0xB4u8, 0x42, // LDY $42,X
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.y, 0x99);

        // ------------------------
        // Absolute X
        // ------------------------
        let mut cpu = Cpu6502::new();

        cpu.x = 0x10;
        cpu.write(0x2000 + 0x10, 0x55);

        let program = [
            0xBCu8, 0x00, 0x20, // LDY $2000,X
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.y, 0x55);
    }
}
