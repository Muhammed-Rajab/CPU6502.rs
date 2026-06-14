#[cfg(test)]
mod tests {
    use super::super::Cpu6502;
    use super::super::flags::Flag;

    #[test]
    fn cpx_sets_flags_correctly() {
        // ------------------------
        // X > operand
        // ------------------------
        let mut cpu = Cpu6502::new();

        cpu.x = 10;

        let program = [
            0xE0, 5, // CPX #$05
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.get_flag(Flag::Carry), true);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);

        // ------------------------
        // X == operand
        // ------------------------
        let mut cpu = Cpu6502::new();

        cpu.x = 10;

        let program = [
            0xE0, 10, // CPX #$0A
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.get_flag(Flag::Carry), true);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);

        // ------------------------
        // X < operand
        // ------------------------
        let mut cpu = Cpu6502::new();

        cpu.x = 5;

        let program = [
            0xE0, 10, // CPX #$0A
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.get_flag(Flag::Carry), false);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), true);
    }

    #[test]
    fn cpy_sets_flags_correctly() {
        // ------------------------
        // Y > operand
        // ------------------------
        let mut cpu = Cpu6502::new();

        cpu.y = 10;

        let program = [
            0xC0, 5, // CPY #$05
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.get_flag(Flag::Carry), true);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);

        // ------------------------
        // Y == operand
        // ------------------------
        let mut cpu = Cpu6502::new();

        cpu.y = 10;

        let program = [
            0xC0, 10, // CPY #$0A
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.get_flag(Flag::Carry), true);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);

        // ------------------------
        // Y < operand
        // ------------------------
        let mut cpu = Cpu6502::new();

        cpu.y = 5;

        let program = [
            0xC0, 10, // CPY #$0A
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.get_flag(Flag::Carry), false);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), true);
    }
}
