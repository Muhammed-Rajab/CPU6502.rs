//-----------------------------------------------
// TESTS                                        |
//-----------------------------------------------

#[cfg(test)]
mod tests {

    use super::super::Cpu6502;
    use super::super::flags::Flag;

    #[test]
    fn flag_set_clear_test() {
        let mut cpu = Cpu6502::new();

        // by default, unused flag must be set
        assert!(cpu.get_flag(Flag::Unused));

        cpu.set_flag(Flag::Carry, true);
        cpu.set_flag(Flag::Zero, true);
        cpu.set_flag(Flag::Interrupt, true);
        cpu.set_flag(Flag::Decimal, true);
        cpu.set_flag(Flag::Break, true);
        cpu.set_flag(Flag::Overflow, true);
        cpu.set_flag(Flag::Negative, true);

        // all flags are set now
        assert_eq!(cpu.status, 0xFF);

        cpu.set_flag(Flag::Carry, false);
        cpu.set_flag(Flag::Zero, false);
        cpu.set_flag(Flag::Interrupt, false);
        cpu.set_flag(Flag::Decimal, false);
        cpu.set_flag(Flag::Break, false);
        cpu.set_flag(Flag::Unused, false); // has no effect
        cpu.set_flag(Flag::Overflow, false);
        cpu.set_flag(Flag::Negative, false);

        // all flags are cleared except unused
        assert_eq!(cpu.status, 0x20);
    }

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

    #[test]
    fn tax_test() {
        let mut cpu = Cpu6502::new();
        let implied_rom = [
            0xA9u8, 0x0a, // LDA #$0a
            0xAA, // TAX
            0xA9u8, 0x00, // LDA #$00
            0xAA, // TAX
            0xA9u8, 0xff, // LDA #$ff
            0xAA, // TAX
        ];
        cpu.load_program_from_memory(&implied_rom);

        cpu.step(); // LDA #$0a
        cpu.step(); // TAX
        assert_eq!(cpu.a, cpu.x);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));

        cpu.step(); // LDA #$00
        cpu.step(); // TAX
        assert_eq!(cpu.a, cpu.x);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(true, cpu.get_flag(Flag::Zero));

        cpu.step(); // LDA #$ff
        cpu.step(); // TAX
        assert_eq!(cpu.a, cpu.x);
        assert_eq!(true, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));
    }

    #[test]
    fn tay_test() {
        let mut cpu = Cpu6502::new();
        let implied_rom = [
            0xA9u8, 0x0a, // LDA #$0a
            0xA8, // TAY
            0xA9u8, 0x00, // LDA #$00
            0xA8, // TAY
            0xA9u8, 0xff, // LDA #$ff
            0xA8, // TAY
        ];
        cpu.load_program_from_memory(&implied_rom);

        cpu.step(); // LDA #$0a
        cpu.step(); // TAY
        assert_eq!(cpu.a, cpu.y);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));

        cpu.step(); // LDA #$00
        cpu.step(); // TAY
        assert_eq!(cpu.a, cpu.y);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(true, cpu.get_flag(Flag::Zero));

        cpu.step(); // LDA #$ff
        cpu.step(); // TAY
        assert_eq!(cpu.a, cpu.y);
        assert_eq!(true, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));
    }

    #[test]
    fn txa_test() {
        let mut cpu = Cpu6502::new();
        let implied_rom = [
            0xA2, 0x0a, // LDX #$0a
            0x8a, // TXA
            0xA2, 0x00, // LDX #$00
            0x8a, // TXA
            0xA2, 0xff, // LDX #$ff
            0x8a, // TXA
        ];
        cpu.load_program_from_memory(&implied_rom);

        cpu.step(); // LDX #$0a
        cpu.step(); // TXA
        assert_eq!(cpu.x, cpu.a);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));

        cpu.step(); // LDX #$00
        cpu.step(); // TXA
        assert_eq!(cpu.x, cpu.a);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(true, cpu.get_flag(Flag::Zero));

        cpu.step(); // LDX #$ff
        cpu.step(); // TXA
        assert_eq!(cpu.x, cpu.a);
        assert_eq!(true, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));
    }

    #[test]
    fn tya_test() {
        let mut cpu = Cpu6502::new();
        let implied_rom = [
            0xA0, 0x0a, // LDY #$0a
            0x98, // TYA
            0xA0, 0x00, // LDY #$00
            0x98, // TYA
            0xA0, 0xff, // LDY #$ff
            0x98, // TYA
        ];
        cpu.load_program_from_memory(&implied_rom);

        cpu.step(); // LDY #$0a
        cpu.step(); // TYA
        assert_eq!(cpu.y, cpu.a);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));

        cpu.step(); // LDY #$00
        cpu.step(); // TYA
        assert_eq!(cpu.y, cpu.a);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(true, cpu.get_flag(Flag::Zero));

        cpu.step(); // LDY #$ff
        cpu.step(); // TYA
        assert_eq!(cpu.y, cpu.a);
        assert_eq!(true, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));
    }

    #[test]
    fn tsx_test() {
        let mut cpu = Cpu6502::new();

        // WARN: needs more test
        let implied_rom = [
            0xBA, // TSX
        ];
        cpu.load_program_from_memory(&implied_rom);

        cpu.step(); // TSX {x = #$FD}
        assert_eq!(cpu.sp, cpu.x);
        assert_eq!(true, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));
    }

    #[test]
    fn txs_test() {
        let mut cpu = Cpu6502::new();
        let implied_rom = [
            0xA2, 0x0a, // LDX #$0a
            0x9a, // TXS
            0xA2, 0x00, // LDX #$00
            0x9a, // TXS
            0xA2, 0xff, // LDX #$ff
            0x9a, // TXS
        ];

        cpu.load_program_from_memory(&implied_rom);

        cpu.step(); // LDX #$0a
        cpu.step(); // TXS
        assert_eq!(cpu.x, cpu.sp);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));

        cpu.step(); // LDX #$00
        cpu.step(); // TXS
        assert_eq!(cpu.x, cpu.sp);
        assert_eq!(false, cpu.get_flag(Flag::Negative));
        assert_eq!(true, cpu.get_flag(Flag::Zero));

        cpu.step(); // LDX #$ff
        cpu.step(); // TXS
        assert_eq!(cpu.x, cpu.sp);
        assert_eq!(true, cpu.get_flag(Flag::Negative));
        assert_eq!(false, cpu.get_flag(Flag::Zero));
    }

    #[test]
    fn pha_test() {
        let mut cpu = Cpu6502::new();
        let implied_rom = [
            0xA9u8, 0xaa, // LDA #$aa
            0x48, // PHA
            0xA9u8, 0xff, // LDA #$ff
            0x48, // PHA
        ];
        cpu.load_program_from_memory(&implied_rom);

        cpu.step(); // LDA #$aa
        cpu.step(); // PHA
        assert_eq!(cpu.sp, 0xFE);
        assert_eq!(cpu.peek_stack(), 0xaa);

        cpu.step(); // LDA #$ff
        cpu.step(); // PHA
        assert_eq!(cpu.sp, 0xFD);
        assert_eq!(cpu.peek_stack(), 0xff);
    }

    #[test]
    fn pla_test() {
        let mut cpu = Cpu6502::new();
        let implied_rom = [
            0xA9u8, 0x0a, // LDA #$0a
            0x48, // PHA
            0xA9, 0xff, // LDA #$ff
            0x48, // PHA
            0xA9, 0x00, // LDA #$00
            0x48, // PHA
            0x68, // PLA {A=#$00, Z=1, N=0}
            0x68, // PLA {A=#$ff, Z=0, N=1}
            0x68, // PLA {A=#$0a, Z=0, N=0}
        ];
        cpu.load_program_from_memory(&implied_rom);

        cpu.step(); // LDA #$0a
        cpu.step(); // PHA
        cpu.step(); // LDA #$ff
        cpu.step(); // PHA
        cpu.step(); // LDA #$00
        cpu.step(); // PHA

        // SP at 0xFC now
        assert_eq!(cpu.sp, 0xFC);

        cpu.step(); // PLA {A=#$00, Z=1, N=0}
        assert_eq!(cpu.sp, 0xFD);
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);

        cpu.step(); // PLA {A=#$ff, Z=0, N=1}
        assert_eq!(cpu.sp, 0xFE);
        assert_eq!(cpu.a, 0xff);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), true);

        cpu.step(); // PLA {A=#$0a, Z=0, N=0}
        assert_eq!(cpu.sp, 0xFF);
        assert_eq!(cpu.a, 0x0a);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn php_test() {
        let mut cpu = Cpu6502::new();
        let implied_rom = [
            0x8u8, // PHP
            0xA9, 0x00,  // LDA #$00
            0x8u8, // PHP
            0xA9, 0xff,  // LDA #$ff
            0x8u8, // PHP
        ];
        cpu.load_program_from_memory(&implied_rom);

        // check
        //   1. sp
        //   2. peek stack

        // Intial sp
        assert_eq!(cpu.sp, 0xFF);

        cpu.step(); // PHP
        assert_eq!(cpu.sp, 0xFE);
        assert_eq!(cpu.peek_stack(), (Flag::Unused as u8) | 0);

        cpu.step(); // LDA #$00
        cpu.step(); // PHP
        assert_eq!(cpu.sp, 0xFD);
        assert_eq!(
            cpu.peek_stack(),
            (Flag::Zero as u8) | (Flag::Unused as u8) | 0
        );

        cpu.step(); // LDA #$ff
        cpu.step(); // PHP
        assert_eq!(cpu.sp, 0xFC);
        assert_eq!(
            cpu.peek_stack(),
            (Flag::Negative as u8) | (Flag::Unused as u8) | 0
        );
    }

    #[test]
    fn plp_test() {
        let mut cpu = Cpu6502::new();
        let implied_rom = [
            0xA9u8, 0xff, // LDA #$ff
            0x48, // PHA
            0x28, // PLP
        ];
        cpu.load_program_from_memory(&implied_rom);

        cpu.step(); // LDA #$ff
        cpu.step(); // PHA
        cpu.step(); // PLP
        assert_eq!(cpu.status, 0xFF);
    }

    #[test]
    fn inx_test() {
        let mut cpu = Cpu6502::new();

        let implied_rom = [
            0xA2u8, 0x00, // LDX #$00
            0xE8, // INX {X=#$01, Z=0, N=0}
            0xA2, 0x7F, // LDX #$7F
            0xE8, // INX {X=#$80, Z=0, N=1}
            0xA2, 0xFF, // LDX #$FF
            0xE8, // INX {X=#$00, Z=1, N=0}
        ];

        cpu.load_program_from_memory(&implied_rom);

        cpu.step(); // LDX #$00
        cpu.step(); // INX
        assert_eq!(cpu.x, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);

        cpu.step(); // LDX #$7F
        cpu.step(); // INX
        assert_eq!(cpu.x, 0x80);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), true);

        cpu.step(); // LDX #$FF
        cpu.step(); // INX
        assert_eq!(cpu.x, 0x00);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn iny_test() {
        let mut cpu = Cpu6502::new();

        let implied_rom = [
            0xA0u8, 0x00, // LDY #$00
            0xC8, // INY {Y=#$01, Z=0, N=0}
            0xA0, 0x7F, // LDY #$7F
            0xC8, // INY {Y=#$80, Z=0, N=1}
            0xA0, 0xFF, // LDY #$FF
            0xC8, // INY {Y=#$00, Z=1, N=0}
        ];

        cpu.load_program_from_memory(&implied_rom);

        cpu.step(); // LDY #$00
        cpu.step(); // INY
        assert_eq!(cpu.y, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);

        cpu.step(); // LDY #$7F
        cpu.step(); // INY
        assert_eq!(cpu.y, 0x80);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), true);

        cpu.step(); // LDY #$FF
        cpu.step(); // INY
        assert_eq!(cpu.y, 0x00);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn dex_test() {
        let mut cpu = Cpu6502::new();

        let implied_rom = [
            0xA2u8, 0x02, // LDX #$02
            0xCA, // DEX {X=#$01, Z=0, N=0}
            0xA2, 0x01, // LDX #$01
            0xCA, // DEX {X=#$00, Z=1, N=0}
            0xA2, 0x00, // LDX #$00
            0xCA, // DEX {X=#$FF, Z=0, N=1}
        ];

        cpu.load_program_from_memory(&implied_rom);

        cpu.step(); // LDX #$02
        cpu.step(); // DEX
        assert_eq!(cpu.x, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);

        cpu.step(); // LDX #$01
        cpu.step(); // DEX
        assert_eq!(cpu.x, 0x00);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);

        cpu.step(); // LDX #$00
        cpu.step(); // DEX
        assert_eq!(cpu.x, 0xFF);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), true);
    }

    #[test]
    fn dey_test() {
        let mut cpu = Cpu6502::new();

        let implied_rom = [
            0xA0u8, 0x02, // LDY #$02
            0x88, // DEY {Y=#$01, Z=0, N=0}
            0xA0, 0x01, // LDY #$01
            0x88, // DEY {Y=#$00, Z=1, N=0}
            0xA0, 0x00, // LDY #$00
            0x88, // DEY {Y=#$FF, Z=0, N=1}
        ];

        cpu.load_program_from_memory(&implied_rom);

        cpu.step(); // LDY #$02
        cpu.step(); // DEY
        assert_eq!(cpu.y, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);

        cpu.step(); // LDY #$01
        cpu.step(); // DEY
        assert_eq!(cpu.y, 0x00);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);

        cpu.step(); // LDY #$00
        cpu.step(); // DEY
        assert_eq!(cpu.y, 0xFF);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), true);
    }

    #[test]
    fn sec_clc_test() {
        let mut cpu = Cpu6502::new();

        let implied_rom = [
            0x38u8, // SEC
            0x18,   // CLC
        ];

        cpu.load_program_from_memory(&implied_rom);

        cpu.step(); // SEC
        assert_eq!(cpu.get_flag(Flag::Carry), true);

        cpu.step(); // CLC
        assert_eq!(cpu.get_flag(Flag::Carry), false);
    }

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
}
