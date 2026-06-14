//-----------------------------------------------
// TESTS                                        |
//-----------------------------------------------

#[cfg(test)]
mod tests {

    use super::super::super::Cpu6502;
    use super::super::super::flags::Flag;

    #[test]
    fn adc_test() {
        let mut cpu = Cpu6502::new();

        let immediate_rom = [
            0xA9u8, 0x05, // LDA #$05
            0x69, 0x03, // ADC #$03 {A=#$08, C=0, Z=0, N=0, V=0}
            0xA9, 0xFF, // LDA #$FF
            0x69, 0x01, // ADC #$01 {A=#$00, C=1, Z=1, N=0, V=0}
            0xA9, 0x7F, // LDA #$7F
            0x69,
            0x01, // ADC #$01 {A=#$81, C=0, Z=0, N=1, V=1} {carry bit of prev wasn't cleared}
            0xA9, 0x80, // LDA #$80
            0x69, 0x80, // ADC #$80 {A=#$00, C=1, Z=1, N=0, V=1}
        ];

        cpu.load_program_from_memory(&immediate_rom);

        cpu.step(); // LDA #$05
        cpu.step(); // ADC #$03
        assert_eq!(cpu.a, 0x08);
        assert_eq!(cpu.get_flag(Flag::Carry), false);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
        assert_eq!(cpu.get_flag(Flag::Overflow), false);

        cpu.step(); // LDA #$FF
        cpu.step(); // ADC #$01
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.get_flag(Flag::Carry), true);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
        assert_eq!(cpu.get_flag(Flag::Overflow), false);

        cpu.step(); // LDA #$7F
        cpu.step(); // ADC #$01 {carry is set by prev adc}
        assert_eq!(cpu.a, 0x81);
        assert_eq!(cpu.get_flag(Flag::Carry), false);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), true);
        assert_eq!(cpu.get_flag(Flag::Overflow), true);

        cpu.step(); // LDA #$80
        cpu.step(); // ADC #$80
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.get_flag(Flag::Carry), true);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
        assert_eq!(cpu.get_flag(Flag::Overflow), true);
    }

    #[test]
    fn sbc_test() {
        let mut cpu = Cpu6502::new();

        let implied_rom = [
            0x38u8, // SEC
            0xA9, 0x08, // LDA #$08
            0xE9, 0x03, // SBC #$03 {A=#$05, C=1, Z=0, N=0, V=0}
            0x38, // SEC
            0xA9, 0x01, // LDA #$01
            0xE9, 0x01, // SBC #$01 {A=#$00, C=1, Z=1, N=0, V=0}
            0x38, // SEC
            0xA9, 0x00, // LDA #$00
            0xE9, 0x01, // SBC #$01 {A=#$FF, C=0, Z=0, N=1, V=0}
            0xA9, 0x01, // LDA #$01
            0xE9, 0x00, // SBC #$00 {A=#$00, C=1, Z=0, N=0, V=1}
            0x38, // SEC
            0xA9, 0x7F, // LDA #$7F
            0xE9, 0xFF, // SBC #$FF {A=#$80, C=0, Z=0, N=1, V=1}
        ];

        cpu.load_program_from_memory(&implied_rom);

        cpu.step(); // SEC
        cpu.step(); // LDA #$08
        cpu.step(); // SBC #$03
        assert_eq!(cpu.a, 0x05);
        assert_eq!(cpu.get_flag(Flag::Carry), true);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
        assert_eq!(cpu.get_flag(Flag::Overflow), false);

        cpu.step(); // SEC
        cpu.step(); // LDA #$01
        cpu.step(); // SBC #$01
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.get_flag(Flag::Carry), true);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
        assert_eq!(cpu.get_flag(Flag::Overflow), false);

        cpu.step(); // SEC
        cpu.step(); // LDA #$00
        cpu.step(); // SBC #$01
        assert_eq!(cpu.a, 0xFF);
        assert_eq!(cpu.get_flag(Flag::Carry), false);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), true);
        assert_eq!(cpu.get_flag(Flag::Overflow), false);
        cpu.step(); // LDA #$01
        cpu.step(); // SBC #$00
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.get_flag(Flag::Carry), true);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
        assert_eq!(cpu.get_flag(Flag::Overflow), false);

        cpu.step(); // SEC
        cpu.step(); // LDA #$7F
        cpu.step(); // SBC #$FF
        assert_eq!(cpu.a, 0x80);
        assert_eq!(cpu.get_flag(Flag::Carry), false);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), true);
        assert_eq!(cpu.get_flag(Flag::Overflow), true);
    }

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
    fn cmp_test() {
        let mut cpu = Cpu6502::new();

        let implied_rom = [
            0xA9u8, 0x05, // LDA #$05
            0xC9, 0x05, // CMP #$05 {C=1, Z=1, N=0}
            0xA9, 0x05, // LDA #$05
            0xC9, 0x03, // CMP #$03 {C=1, Z=0, N=0}
            0xA9, 0x03, // LDA #$03
            0xC9, 0x05, // CMP #$05 {C=0, Z=0, N=1}
        ];

        cpu.load_program_from_memory(&implied_rom);

        cpu.step(); // LDA #$05
        cpu.step(); // CMP #$05
        assert_eq!(cpu.a, 0x05);
        assert_eq!(cpu.get_flag(Flag::Carry), true);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);

        cpu.step(); // LDA #$05
        cpu.step(); // CMP #$03
        assert_eq!(cpu.a, 0x05);
        assert_eq!(cpu.get_flag(Flag::Carry), true);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);

        cpu.step(); // LDA #$03
        cpu.step(); // CMP #$05
        assert_eq!(cpu.a, 0x03);
        assert_eq!(cpu.get_flag(Flag::Carry), false);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), true);
    }

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
