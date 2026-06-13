#[cfg(test)]
mod tests {

    use super::super::Cpu6502;
    use super::super::flags::Flag;

    #[test]
    fn absolute_mode_reads_correct_memory() {
        let mut cpu = Cpu6502::new();

        let program = [
            0xADu8, 0x34, 0x12, // LDA $1234
        ];

        cpu.write(0x1234, 0x42);

        cpu.load_program_from_memory(&program);

        cpu.step(); // LDA $1234
        assert_eq!(cpu.a, 0x42);
    }

    #[test]
    fn zeropage_reads_correct_memory() {
        let mut cpu = Cpu6502::new();

        let program = [
            0xA5, 0xAA, // LDA $AA
        ];

        cpu.write(0x00AA, 0x99);

        cpu.load_program_from_memory(&program);

        cpu.step(); // LDA $AA
        assert_eq!(cpu.a, 0x99);
    }

    #[test]
    fn zeropage_x_addressing_wraps_and_loads_correct_value() {
        let mut cpu = Cpu6502::new();

        cpu.x = 0x05;

        // base address = 0xFE, + X = 0x03 (wraps in zero page)
        cpu.memory[0x0003] = 0x42;

        let program = [
            0xB5, 0xFE, // LDA $FE,X
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.a, 0x42);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn immediate_loads_direct_value() {
        let mut cpu = Cpu6502::new();

        let program = [
            0xA9, 0x77, // LDA #$77
        ];

        cpu.load_program_from_memory(&program);

        cpu.step(); // LDA #$77
        assert_eq!(cpu.a, 0x77);
    }
}
