#[derive(Copy, Clone)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Relative,
    Implied,
}
