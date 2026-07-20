#[cfg(test)]
mod tests {

    use super::super::super::Cpu6502;
    use super::super::super::flags::Flag;

    #[test]
    fn klaus_functional_test() {
        let mut cpu = Cpu6502::new();

        let program = std::fs::read("test_binaries/6502_functional_test.bin").unwrap();

        cpu.load_program_from_memory(&program);
        cpu.pc = 0x0400;

        let mut last_pc = 0xFFFF;

        loop {
            if cpu.pc == 0x3472 {
                println!(
                    "A={:02X} M={:02X} C={}",
                    cpu.a,
                    cpu.read(0x000E),
                    cpu.get_flag(Flag::Carry)
                );
            }
            //
            // if cpu.pc == 0x3475 {
            //     println!(
            //         "A={:02X} M0D={:02X} M0E={:02X} M0F={:02X} P={:02X}",
            //         cpu.a,
            //         cpu.read(0x000D),
            //         cpu.read(0x000E),
            //         cpu.read(0x000F),
            //         cpu.status
            //     );
            // }

            cpu.step();

            if cpu.pc == last_pc {
                // cpu.hexdump(0x00, 0xFF);
                // cpu.hexdump(0x3450, 0xFF);
                assert_eq!(cpu.pc, 0x3469);
                break;
            }

            last_pc = cpu.pc;
        }
    }
}
