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
        cpu.write(0x0003, 0x42);

        let program = [
            0xB5, 0xFE, // LDA $FE,X
        ];

        cpu.load_program_from_memory(&program);
        cpu.step(); // LDA $FE,X

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

    #[test]
    fn absolute_x_addressing_loads_correct_memory() {
        let mut cpu = Cpu6502::new();

        cpu.x = 0x10;

        cpu.write(0x1234 + 0x10, 0x42);

        let program = [
            0xBD, 0x34, 0x12, // LDA $1234,X
        ];

        cpu.load_program_from_memory(&program);

        cpu.step(); // LDA $1234,X
        assert_eq!(cpu.a, 0x42);
    }

    #[test]
    fn absolute_x_cross_page_reads_correct_memory() {
        let mut cpu = Cpu6502::new();

        cpu.x = 0x05;

        cpu.write(0x10FF + 0x05, 0x99);

        let program = [
            0xBD, 0xFF, 0x10, // LDA $10FF,X
        ];

        cpu.load_program_from_memory(&program);

        cpu.step(); // LDA $10FF,X
        assert_eq!(cpu.a, 0x99);
    }

    #[test]
    fn absolute_y_addressing_loads_correct_memory() {
        let mut cpu = Cpu6502::new();

        cpu.y = 0x10;

        cpu.write(0x1234 + 0x10, 0x42);

        let program = [
            0xB9u8, 0x34, 0x12, // LDA $1234,Y
        ];

        cpu.load_program_from_memory(&program);

        cpu.step(); // LDA $1234,Y
        assert_eq!(cpu.a, 0x42);
    }

    #[test]
    fn absolute_y_cross_page_reads_correct_memory() {
        let mut cpu = Cpu6502::new();

        cpu.y = 0x05;

        cpu.write(0x10FF + 0x05, 0x99);

        let program = [
            0xB9u8, 0xFF, 0x10, // LDA $10FF,Y
        ];

        cpu.load_program_from_memory(&program);

        cpu.step(); // LDA $10FF,Y
        assert_eq!(cpu.a, 0x99);
    }

    #[test]
    fn indexed_indirect_x_fetches_correct_value() {
        let mut cpu = Cpu6502::new();

        cpu.x = 0x04;

        // base = 0x20, (0x20 + X) = 0x24 → pointer stored in zero page
        cpu.write(0x0024, 0x00); // low byte
        cpu.write(0x0025, 0x80); // high byte → address = 0x8000

        cpu.write(0x8000, 0x42);

        let program = [
            0xA1u8, 0x20, // LDA ($20,X)
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.a, 0x42);
    }

    #[test]
    fn indexed_indirect_x_wraps_zero_page_pointer() {
        let mut cpu = Cpu6502::new();

        cpu.x = 0xff;

        println!("X: 0x{:02X}", cpu.x);

        // base = 0x70, (0x70 + 0xff) = 0x6f (wrap)
        cpu.write(0x006f, 0x34); // low byte
        cpu.write(0x0070, 0x12); // high byte (wrap in zero page)

        cpu.hexdump(0x0000, 0x100);

        cpu.write(0x1234, 0x99);

        let program = [
            0xA1u8, 0x70, // LDA ($70,X)
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.a, 0x99);
    }

    #[test]
    fn indirect_indexed_y_addressing_loads_correct_memory() {
        let mut cpu = Cpu6502::new();

        cpu.y = 0x10;

        // pointer stored in zero page at $0034/$0035
        cpu.write(0x0034, 0x00); // low byte
        cpu.write(0x0035, 0x80); // high byte → pointer = 0x8000

        // final address = 0x8000 + 0x10 = 0x8010
        cpu.write(0x8010, 0x42);

        let program = [
            0xB1u8, 0x34, // LDA ($34),Y
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.a, 0x42);
    }

    #[test]
    fn indirect_indexed_y_cross_page() {
        let mut cpu = Cpu6502::new();

        cpu.y = 0x05;

        // pointer = 0x10FF
        cpu.write(0x00AA, 0xFF); // low
        cpu.write(0x00AB, 0x10); // high

        // 0x10FF + 0x05 = 0x1104
        cpu.write(0x1104, 0x99);

        let program = [
            0xB1u8, 0xAA, // LDA ($AA),Y
        ];

        cpu.load_program_from_memory(&program);
        cpu.step();

        assert_eq!(cpu.a, 0x99);
    }
}
