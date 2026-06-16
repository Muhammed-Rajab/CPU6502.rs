#[cfg(test)]
mod tests {

    use super::super::super::Cpu6502;

    #[test]
    fn klaus_functional_test() {
        let mut cpu = Cpu6502::new();

        let program = std::fs::read("test_binaries/6502_functional_test.bin").unwrap();

        cpu.load_program_from_memory(&program);
        cpu.pc = 0x0400;

        // cpu.hexdump(0x0660, 0xFF);

        let mut last_pc = 0xFFFF;

        loop {
            cpu.step();

            if cpu.pc == last_pc {
                assert_eq!(cpu.pc, 0x3469);
                break;
            }

            last_pc = cpu.pc;
        }
    }
}
