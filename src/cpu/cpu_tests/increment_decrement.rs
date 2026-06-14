#[cfg(test)]
mod tests {

    use super::super::super::Cpu6502;
    use super::super::super::flags::Flag;

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
}
