#[cfg(test)]
mod tests {

    use super::super::Cpu6502;
    use super::super::flags::Flag;

    #[test]
    fn inc_updates_memory_and_flags_correctly() {
        // ------------------------
        // Normal increment
        // ------------------------
        let mut cpu = Cpu6502::new();

        cpu.write(0x1234, 0x41);

        let program = [
            0xEE, 0x34, 0x12, // INC $1234
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.read(0x1234), 0x42);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);

        // ------------------------
        // Wrap to zero
        // ------------------------
        let mut cpu = Cpu6502::new();

        cpu.write(0x1234, 0xFF);

        let program = [
            0xEE, 0x34, 0x12, // INC $1234
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.read(0x1234), 0x00);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);

        // ------------------------
        // Set negative
        // ------------------------
        let mut cpu = Cpu6502::new();

        cpu.write(0x1234, 0x7F);

        let program = [
            0xEE, 0x34, 0x12, // INC $1234
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.read(0x1234), 0x80);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), true);
    }

    #[test]
    fn dec_updates_memory_and_flags_correctly() {
        // ------------------------
        // Normal decrement
        // ------------------------
        let mut cpu = Cpu6502::new();

        cpu.write(0x1234, 0x42);

        let program = [
            0xCE, 0x34, 0x12, // DEC $1234
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.read(0x1234), 0x41);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);

        // ------------------------
        // Decrement to zero
        // ------------------------
        let mut cpu = Cpu6502::new();

        cpu.write(0x1234, 0x01);

        let program = [
            0xCE, 0x34, 0x12, // DEC $1234
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.read(0x1234), 0x00);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);

        // ------------------------
        // Wrap and set negative
        // ------------------------
        let mut cpu = Cpu6502::new();

        cpu.write(0x1234, 0x00);

        let program = [
            0xCE, 0x34, 0x12, // DEC $1234
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.read(0x1234), 0xFF);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), true);
    }
}
