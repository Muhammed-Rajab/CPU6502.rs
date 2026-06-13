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
}
