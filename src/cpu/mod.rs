/*
 *
 *-----------------
 * SPECIFICATIONS |
 *-----------------
 *
 * 8-bit little endian
 *
 *---------------
 * STATUS FLAGS |
 *---------------
 *
 * 7 6 5 4  3 2 1 0
 * - - - -  - - - -
 * N V 1 B  D I Z C
 *
 * Carry (C) -> Set if last operation an overflow from
 *              bit 7 of the result (like in `ADC`) or an
 *              underflow (like in `SBC`).
 *
 *              Arithmetic, comparison, and logical shifts.
 *
 *              Explicitly set using the `SEC` instruction and
 *              cleared with `CLC` instruction.
 *
 * Zero (Z) -> Set if the result of last operation was zero.
 *
 * Interrupt Disable (I) -> Set if `SEI` instruction was executed.
 *                          When set, the processor will not
 *                          respond to interrupts from devices until
 *                          it is cleared by `CLI` instruction.
 *
 * Decimal Mode (D) -> When set, the processor obeys Binary Coded Decimal
 *                      arithmetic during addition and subtraction (not sure
 *                      what that means, lol). Can be set using `SED` and
 *                      cleared with `CLD`.
 *
 * Break (B) -> Set when BRK instruction has been executed and an interrupt
 *              has been generated to process it.
 *
 * Overflow (V) -> Set during arithmetic operations if the results has yielded
 *                  an invalid 2's complement result (eg: adding positives and
 *                  end up getting negative, 64 + 64 = -128
 *
 *                  Can't trigger overflow if two numbers you are adding have
 *                  different signs.
 *
 *                  Carry is for unsigned.
 *                  Overflow is for signed.
 *
 * Negative (N) -> Set if the result of the last operation had bit 7 set to one.
 *
 *----------------
 * MEMORY LAYOUT |
 *----------------
 *
 * $0000-$00FF = Zero Page (256 bytes)
 * $0100-$01FF = Stack (second page, 256 bytes)
 * $0200-$FFF9 = General memory
 * $FFFA-$FFFB = NMI Vector
 * $FFFC-$FFFD = Reset Vector
 * $FFFE-$FFFF = IRQ/BRK Vector
 *
 * SP starts at $FF, and decreases when pushed into and vice versa.
 * SP wraps since it's an 8-bit register. PHA at SP=$00 stores value of A to $00 and decreases
 * it by one, wrapping it, resulting in SP=$FF.
 * For debugging purposes, our PC starts at $0600.
 * */

// DECLARATION
mod addressing_modes;
mod cpu_tests;
mod debug;
mod dispatch;
mod flags;
mod instructions;
mod memory;

pub const START_PC_ADDRESS: u16 = 0x0000;

pub struct Cpu6502 {
    a: u8,               // A ccumulator
    x: u8,               // X register
    y: u8,               // Y register
    sp: u8,              // Stack pointer
    pc: u16,             // Program Counter
    status: u8,          // Status Flags
    memory: [u8; 65536], // Memory
}

impl Cpu6502 {
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            sp: 0xFF,                          // Reset value of SP
            pc: START_PC_ADDRESS,              // Should be loaded from Reset Vector
            status: flags::Flag::Unused as u8, // By default, it's on.
            memory: [0; 65536],                // Zero initialised array
        }
    }
}
