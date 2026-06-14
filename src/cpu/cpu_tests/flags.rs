#[cfg(test)]
mod tests {

    use super::super::super::Cpu6502;
    use super::super::super::flags::Flag;

    #[test]
    fn flag_set_clear_test() {
        let mut cpu = Cpu6502::new();

        // by default, unused flag must be set
        assert!(cpu.get_flag(Flag::Unused));

        cpu.set_flag(Flag::Carry, true);
        cpu.set_flag(Flag::Zero, true);
        cpu.set_flag(Flag::Interrupt, true);
        cpu.set_flag(Flag::Decimal, true);
        cpu.set_flag(Flag::Break, true);
        cpu.set_flag(Flag::Overflow, true);
        cpu.set_flag(Flag::Negative, true);

        // all flags are set now
        assert_eq!(cpu.status, 0xFF);

        cpu.set_flag(Flag::Carry, false);
        cpu.set_flag(Flag::Zero, false);
        cpu.set_flag(Flag::Interrupt, false);
        cpu.set_flag(Flag::Decimal, false);
        cpu.set_flag(Flag::Break, false);
        cpu.set_flag(Flag::Unused, false); // has no effect
        cpu.set_flag(Flag::Overflow, false);
        cpu.set_flag(Flag::Negative, false);

        // all flags are cleared except unused
        assert_eq!(cpu.status, 0x20);
    }

    #[test]
    fn sec_clc_test() {
        let mut cpu = Cpu6502::new();

        let implied_rom = [
            0x38u8, // SEC
            0x18,   // CLC
        ];

        cpu.load_program_from_memory(&implied_rom);

        cpu.step(); // SEC
        assert_eq!(cpu.get_flag(Flag::Carry), true);

        cpu.step(); // CLC
        assert_eq!(cpu.get_flag(Flag::Carry), false);
    }
}
