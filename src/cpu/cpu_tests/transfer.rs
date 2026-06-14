#[cfg(test)]
mod tests {

    use super::super::super::Cpu6502;
    use super::super::super::flags::Flag;

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
}
