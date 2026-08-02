mod cpu;

use cpu::Cpu6502;

fn main() {
    let mut cpu = Cpu6502::new();
    let rom = [
        0xa9, 0x05, // LDA #5
        0xa9, 0x00, // LDA #0
    ];
    cpu.load_program_from_memory(&rom);

    cpu.step(); // to stop all those fucking warnings.
    cpu.hexdump(cpu::START_PC_ADDRESS, 0x00ff);
}
