#[cfg(test)]
mod tests {
    use super::super::super::Cpu6502;
    use super::super::super::flags::Flag;

    #[test]
    fn brk_jumps_to_irq_vector_and_sets_interrupt_flag() {
        let mut cpu = Cpu6502::new();

        cpu.write(0xFFFE, 0x34);
        cpu.write(0xFFFF, 0x12);

        let program = [
            0x00, // BRK
        ];

        cpu.load_program_from_memory(&program);

        cpu.step();

        assert_eq!(cpu.pc, 0x1234);
        assert!(cpu.get_flag(Flag::Interrupt));
    }

    #[test]
    fn brk_pushes_return_address_and_status() {
        let mut cpu = Cpu6502::new();

        cpu.write(0xFFFE, 0x34);
        cpu.write(0xFFFF, 0x12);

        let initial_sp = cpu.sp;

        let program = [
            0x00, // BRK
        ];

        cpu.load_program_from_memory(&program);

        cpu.step();

        assert_eq!(cpu.sp, initial_sp.wrapping_sub(3));

        assert_eq!(cpu.read(0x01FF), 0x06); // PC high
        assert_eq!(cpu.read(0x01FE), 0x02); // PC low (assuming BRK at $0600)
        assert!(cpu.read(0x01FD) & 0x10 != 0); // Break flag
    }

    #[test]
    fn brk_rti_roundtrip() {
        let mut cpu = Cpu6502::new();

        cpu.write(0xFFFE, 0x06);
        cpu.write(0xFFFF, 0x06); // IRQ/BRK vector -> $0606

        let program = [
            0x00, // $0600: BRK
            0xEA, // $0601: BRK padding byte (ignored)
            0xA9, 0x42, // $0602: LDA #$42 (should execute after RTI)
            0xEA, // $0604
            0xEA, // $0605
            0x40, // $0606: RTI
        ];

        cpu.load_program_from_memory(&program);

        cpu.step(); // BRK
        assert_eq!(cpu.pc, 0x0606);

        cpu.step(); // RTI
        assert_eq!(cpu.pc, 0x0602);

        cpu.step(); // LDA #$42
        assert_eq!(cpu.a, 0x42);
    }
}
