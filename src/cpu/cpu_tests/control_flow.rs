#[cfg(test)]
mod tests {

    use super::super::super::Cpu6502;

    #[test]
    fn jmp_absolute() {
        let mut cpu = Cpu6502::new();

        let program = [
            0x4C, 0x34, 0x12, // JMP $1234
        ];

        cpu.load_program_from_memory(&program);

        cpu.step();

        assert_eq!(cpu.pc, 0x1234);
    }

    #[test]
    fn jmp_indirect_page_wrap_bug() {
        let mut cpu = Cpu6502::new();

        // Pointer at $12FF
        cpu.write(0x12FF, 0x34); // low byte of target

        // Real 6502 bug: high byte comes from $1200, not $1300
        cpu.write(0x1200, 0x12);
        cpu.write(0x1300, 0x56); // should be ignored

        let program = [
            0x6C, 0xFF, 0x12, // JMP ($12FF)
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.pc, 0x1234);
    }
}
