#[derive(Copy, Clone)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    Relative,
    Absolute,
    Implied,
}
