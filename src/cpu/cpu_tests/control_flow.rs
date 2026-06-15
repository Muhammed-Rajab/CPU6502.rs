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

    #[test]
    fn jsr_rts_roundtrip() {
        let mut cpu = Cpu6502::new();

        let program = [
            0x20u8, 0x06, 0x06, // JSR $0606
            0xA2, 0xFF, // LDX #$ff
            0x00, // BRK
            0xA9, 0x10, // LDA #$10
            0x60, // RTS
        ];

        cpu.load_program_from_memory(&program);

        assert_eq!(cpu.pc, 0x0600);
        cpu.step(); // JSR $0606
        assert_eq!(cpu.pc, 0x0606);

        cpu.step(); // LDA #$10
        cpu.step(); // RTS

        assert_eq!(cpu.pc, 0x0600 + 3); // now at $0603

        cpu.step(); // LDX #$ff

        assert_eq!(cpu.a, 0x10);
        assert_eq!(cpu.x, 0xff);

        // cpu.step(); // BRK (error)
    }
}
