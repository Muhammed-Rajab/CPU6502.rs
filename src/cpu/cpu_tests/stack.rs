#[cfg(test)]
mod tests {

    use super::super::super::Cpu6502;
    use super::super::super::flags::Flag;

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
}
