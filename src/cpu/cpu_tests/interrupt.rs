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
}
