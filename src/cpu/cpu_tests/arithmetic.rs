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
}
