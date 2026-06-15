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
}
