#[derive(Copy, Clone)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    Relative,
    Absolute,
    Implied,
}
