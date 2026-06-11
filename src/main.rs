mod cpu;

use cpu::Cpu6502;
use cpu::START_PC_ADDRESS;

fn main() {
    let mut cpu = Cpu6502::new();
    let rom = [
        0xa9, 0x05, // LDA #5
        0xa9, 0x00, // LDA #0
    ];
    cpu.load_program_from_memory(&rom);

    cpu.step(); // to stop all those fucking warnings.
    cpu.hexdump(START_PC_ADDRESS, 0x00ff);

    // test byte construction
    let low = 0b11111111u16;
    let high = 0b10001001u16;
    let combined = (high << 8) | low;

    println!("low : {:#018b}", low);
    println!("high: {:#018b}", high);
    println!("res : {:#018b}", combined);

    // stack address forming
    // let sp = 0x0100u16 | (cpu.sp as u16);
    // println!("sp: ${:04X}", sp);
}
