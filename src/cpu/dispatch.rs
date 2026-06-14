//-----------------------------------------------
// OPERATION                                    |
//-----------------------------------------------

use std::sync::LazyLock;

use super::Cpu6502;
use super::addressing_modes::AddressingMode;

#[derive(Clone, Copy)]
enum Operation {
    // LOAD
    LDA,
    LDX,
    LDY,

    // STORE
    STA,
    STX,
    STY,

    // TRANSFER
    TAX,
    TAY,
    TXA,
    TYA,
    TSX,
    TXS,

    // STACK
    PHA,
    PLA,
    PHP,
    PLP,

    // INCREMENT/DECREMENT
    INX,
    INY,
    DEX,
    DEY,

    // FLAGS
    CLC,
    SEC,

    // ARITHMETIC
    ADC,
    SBC,

    // LOGICAL
    AND,
    ORA,
    EOR,

    // BRANCHING/COMPARISON
    CMP,
    BEQ,
    BNE,

    // SPECIAL CASES
    Invalid,
}

struct Instruction {
    mnemonic: &'static str,
    operation: Operation,
    mode: AddressingMode,
    bytes: u8,
    cycles: u8,
}

const fn instr(
    mnemonic: &'static str,
    operation: Operation,
    mode: AddressingMode,
    bytes: u8,
    cycles: u8,
) -> Instruction {
    Instruction {
        mnemonic,
        operation,
        mode,
        bytes,
        cycles,
    }
}

const INVALID: Instruction = instr("???", Operation::Invalid, AddressingMode::Implied, 1, 0);

static OPCODE_TABLE: LazyLock<[Instruction; 256]> = LazyLock::new(|| {
    let mut table = [INVALID; 256];

    //-------
    // LOAD |
    //-------

    // LDA
    table[0xA9] = instr("LDA", Operation::LDA, AddressingMode::Immediate, 2, 2);
    table[0xA5] = instr("LDA", Operation::LDA, AddressingMode::ZeroPage, 2, 3);
    table[0xB5] = instr("LDA", Operation::LDA, AddressingMode::ZeroPageX, 2, 4);
    table[0xAD] = instr("LDA", Operation::LDA, AddressingMode::Absolute, 3, 4);
    table[0xBD] = instr("LDA", Operation::LDA, AddressingMode::AbsoluteX, 3, 4);
    table[0xB9] = instr("LDA", Operation::LDA, AddressingMode::AbsoluteY, 3, 4);
    table[0xA1] = instr("LDA", Operation::LDA, AddressingMode::IndexedIndirect, 2, 6);
    table[0xB1] = instr("LDA", Operation::LDA, AddressingMode::IndirectIndexed, 2, 5);

    // LDX
    table[0xA2] = instr("LDX", Operation::LDX, AddressingMode::Immediate, 2, 2);
    table[0xA6] = instr("LDX", Operation::LDX, AddressingMode::ZeroPage, 2, 2);
    table[0xB6] = instr("LDX", Operation::LDX, AddressingMode::ZeroPageY, 2, 2);
    table[0xAE] = instr("LDX", Operation::LDX, AddressingMode::Absolute, 2, 2);
    table[0xBE] = instr("LDX", Operation::LDX, AddressingMode::AbsoluteY, 2, 2);

    // LDY
    table[0xA0] = instr("LDY", Operation::LDY, AddressingMode::Immediate, 2, 2);
    table[0xA4] = instr("LDY", Operation::LDY, AddressingMode::ZeroPage, 2, 2);
    table[0xB4] = instr("LDY", Operation::LDY, AddressingMode::ZeroPageX, 2, 2);
    table[0xAC] = instr("LDY", Operation::LDY, AddressingMode::Absolute, 2, 2);
    table[0xBC] = instr("LDY", Operation::LDY, AddressingMode::AbsoluteX, 2, 2);

    //--------
    // STORE |
    //--------

    // STA
    table[0x85] = instr("STA", Operation::STA, AddressingMode::ZeroPage, 2, 3);
    table[0x95] = instr("STA", Operation::STA, AddressingMode::ZeroPageX, 2, 3);
    table[0x8D] = instr("STA", Operation::STA, AddressingMode::Absolute, 3, 4);
    table[0x9D] = instr("STA", Operation::STA, AddressingMode::AbsoluteX, 3, 4);
    table[0x99] = instr("STA", Operation::STA, AddressingMode::AbsoluteY, 3, 4);
    table[0x81] = instr("STA", Operation::STA, AddressingMode::IndexedIndirect, 3, 4);
    table[0x91] = instr("STA", Operation::STA, AddressingMode::IndirectIndexed, 3, 4);

    // STX
    table[0x86] = instr("STX", Operation::STX, AddressingMode::ZeroPage, 2, 3);
    table[0x96] = instr("STX", Operation::STX, AddressingMode::ZeroPageY, 2, 3);
    table[0x8E] = instr("STX", Operation::STX, AddressingMode::Absolute, 3, 4);

    // STY
    table[0x8C] = instr("STY", Operation::STY, AddressingMode::Absolute, 3, 4);
    table[0x84] = instr("STY", Operation::STY, AddressingMode::ZeroPage, 2, 3);

    //-----------
    // TRANSFER |
    //-----------

    // TAX
    table[0xAA] = instr("TAX", Operation::TAX, AddressingMode::Implied, 1, 2);

    // TAY
    table[0xA8] = instr("TAY", Operation::TAY, AddressingMode::Implied, 1, 2);

    // TXA
    table[0x8A] = instr("TXA", Operation::TXA, AddressingMode::Implied, 1, 2);

    // TYA
    table[0x98] = instr("TYA", Operation::TYA, AddressingMode::Implied, 1, 2);

    // TSX
    table[0xBA] = instr("TSX", Operation::TSX, AddressingMode::Implied, 1, 2);

    // TXS
    table[0x9A] = instr("TXS", Operation::TXS, AddressingMode::Implied, 1, 2);

    //--------
    // STACK |
    //--------

    // PHA
    table[0x48] = instr("PHA", Operation::PHA, AddressingMode::Implied, 1, 3);

    // PLA
    table[0x68] = instr("PLA", Operation::PLA, AddressingMode::Implied, 1, 4);

    // PHP
    table[0x08] = instr("PHP", Operation::PHP, AddressingMode::Implied, 1, 4);

    // PLP
    table[0x28] = instr("PLP", Operation::PLP, AddressingMode::Implied, 1, 4);

    //----------------------
    // INCREMENT/DECREMENT |
    //----------------------

    // INX
    table[0xE8] = instr("INX", Operation::INX, AddressingMode::Implied, 1, 4);

    // INY
    table[0xC8] = instr("INY", Operation::INY, AddressingMode::Implied, 1, 4);

    // DEX
    table[0xCA] = instr("DEX", Operation::DEX, AddressingMode::Implied, 1, 4);

    // DEY
    table[0x88] = instr("DEY", Operation::DEY, AddressingMode::Implied, 1, 4);

    //--------
    // FLAGS |
    //--------

    // CLC
    table[0x18] = instr("CLC", Operation::CLC, AddressingMode::Implied, 1, 4);

    // SEC
    table[0x38] = instr("SEC", Operation::SEC, AddressingMode::Implied, 1, 4);

    //-------------
    // ARITHMETIC |
    //-------------

    // ADC
    table[0x69] = instr("ADC", Operation::ADC, AddressingMode::Immediate, 1, 4);
    table[0x65] = instr("ADC", Operation::ADC, AddressingMode::ZeroPage, 1, 4);
    table[0x75] = instr("ADC", Operation::ADC, AddressingMode::ZeroPageX, 1, 4);
    table[0x6D] = instr("ADC", Operation::ADC, AddressingMode::Absolute, 1, 4);
    table[0x7D] = instr("ADC", Operation::ADC, AddressingMode::AbsoluteX, 1, 4);
    table[0x79] = instr("ADC", Operation::ADC, AddressingMode::AbsoluteY, 1, 4);
    table[0x61] = instr("ADC", Operation::ADC, AddressingMode::IndexedIndirect, 1, 4);
    table[0x71] = instr("ADC", Operation::ADC, AddressingMode::IndirectIndexed, 1, 4);

    // SBC
    table[0xE9] = instr("SBC", Operation::SBC, AddressingMode::Immediate, 1, 4);
    table[0xE5] = instr("SBC", Operation::SBC, AddressingMode::ZeroPage, 1, 4);
    table[0xF5] = instr("SBC", Operation::SBC, AddressingMode::ZeroPageX, 1, 4);
    table[0xED] = instr("SBC", Operation::SBC, AddressingMode::Absolute, 1, 4);
    table[0xFD] = instr("SBC", Operation::SBC, AddressingMode::AbsoluteX, 1, 4);
    table[0xF9] = instr("SBC", Operation::SBC, AddressingMode::AbsoluteY, 1, 4);
    table[0xE1] = instr("SBC", Operation::SBC, AddressingMode::IndexedIndirect, 1, 4);
    table[0xF1] = instr("SBC", Operation::SBC, AddressingMode::IndirectIndexed, 1, 4);

    //----------
    // LOGICAL |
    //----------

    // AND
    table[0x29] = instr("AND", Operation::AND, AddressingMode::Immediate, 1, 4);
    table[0x25] = instr("AND", Operation::AND, AddressingMode::ZeroPage, 1, 4);
    table[0x35] = instr("AND", Operation::AND, AddressingMode::ZeroPageX, 1, 4);
    table[0x2D] = instr("AND", Operation::AND, AddressingMode::Absolute, 1, 4);
    table[0x3D] = instr("AND", Operation::AND, AddressingMode::AbsoluteX, 1, 4);
    table[0x39] = instr("AND", Operation::AND, AddressingMode::AbsoluteY, 1, 4);
    table[0x21] = instr("AND", Operation::AND, AddressingMode::IndexedIndirect, 1, 4);
    table[0x31] = instr("AND", Operation::AND, AddressingMode::IndirectIndexed, 1, 4);

    // ORA
    table[0x09] = instr("ORA", Operation::ORA, AddressingMode::Immediate, 1, 4);
    table[0x05] = instr("ORA", Operation::ORA, AddressingMode::ZeroPage, 1, 4);
    table[0x15] = instr("ORA", Operation::ORA, AddressingMode::ZeroPageX, 1, 4);
    table[0x0D] = instr("ORA", Operation::ORA, AddressingMode::Absolute, 1, 4);
    table[0x1D] = instr("ORA", Operation::ORA, AddressingMode::AbsoluteX, 1, 4);
    table[0x19] = instr("ORA", Operation::ORA, AddressingMode::AbsoluteY, 1, 4);
    table[0x01] = instr("ORA", Operation::ORA, AddressingMode::IndexedIndirect, 1, 4);
    table[0x11] = instr("ORA", Operation::ORA, AddressingMode::IndirectIndexed, 1, 4);

    // EOR
    table[0x49] = instr("EOR", Operation::EOR, AddressingMode::Immediate, 1, 4);
    table[0x45] = instr("EOR", Operation::EOR, AddressingMode::ZeroPage, 1, 4);
    table[0x55] = instr("EOR", Operation::EOR, AddressingMode::ZeroPageX, 1, 4);
    table[0x4D] = instr("EOR", Operation::EOR, AddressingMode::Absolute, 1, 4);
    table[0x5D] = instr("EOR", Operation::EOR, AddressingMode::AbsoluteX, 1, 4);
    table[0x59] = instr("EOR", Operation::EOR, AddressingMode::AbsoluteY, 1, 4);
    table[0x41] = instr("EOR", Operation::EOR, AddressingMode::IndexedIndirect, 1, 4);
    table[0x51] = instr("EOR", Operation::EOR, AddressingMode::IndirectIndexed, 1, 4);

    //-----------------------
    // BRANCHING/COMPARISON |
    //-----------------------

    // CMP
    table[0xC9] = instr("CMP", Operation::CMP, AddressingMode::Immediate, 1, 4);
    table[0xC5] = instr("CMP", Operation::CMP, AddressingMode::ZeroPage, 1, 4);
    table[0xD5] = instr("CMP", Operation::CMP, AddressingMode::ZeroPageX, 1, 4);
    table[0xCD] = instr("CMP", Operation::CMP, AddressingMode::Absolute, 1, 4);
    table[0xDD] = instr("CMP", Operation::CMP, AddressingMode::AbsoluteX, 1, 4);
    table[0xD9] = instr("CMP", Operation::CMP, AddressingMode::AbsoluteY, 1, 4);
    table[0xC1] = instr("CMP", Operation::CMP, AddressingMode::IndexedIndirect, 1, 4);
    table[0xD1] = instr("CMP", Operation::CMP, AddressingMode::IndirectIndexed, 1, 4);

    // BEQ
    table[0xF0] = instr("BEQ", Operation::BEQ, AddressingMode::Relative, 1, 4);

    // BNE
    table[0xD0] = instr("BNE", Operation::BNE, AddressingMode::Relative, 1, 4);

    table
});

impl Cpu6502 {
    pub fn step(&mut self) {
        let opcode = self.fetch_byte();

        println!("PC=${:04X}, OPCODE=${:02X}", self.pc, opcode);

        let instruction = &OPCODE_TABLE[opcode as usize];

        self.execute(instruction);
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction.operation {
            //-------
            // LOAD |
            //-------
            Operation::LDA => {
                let value = self.fetch_value(instruction.mode);
                self.lda(value);
            }

            Operation::LDX => {
                let value = self.fetch_value(instruction.mode);
                self.ldx(value);
            }

            Operation::LDY => {
                let value = self.fetch_value(instruction.mode);
                self.ldy(value);
            }

            //--------
            // STORE |
            //--------
            Operation::STA => {
                // get address
                let addr = self.fetch_addr(instruction.mode);
                self.sta(addr);
            }

            Operation::STX => {
                // get address
                let addr = self.fetch_addr(instruction.mode);
                self.stx(addr);
            }

            Operation::STY => {
                // get address
                let addr = self.fetch_addr(instruction.mode);
                self.sty(addr);
            }

            //-----------
            // TRANSFER |
            //-----------
            Operation::TAX => {
                // only implied
                self.tax();
            }

            Operation::TAY => {
                // only implied
                self.tay();
            }

            Operation::TXA => {
                // only implied
                self.txa();
            }

            Operation::TYA => {
                // only implied
                self.tya();
            }

            Operation::TSX => {
                // only implied
                self.tsx();
            }

            Operation::TXS => {
                // only implied
                self.txs();
            }

            //--------
            // STACK |
            //--------
            Operation::PHA => {
                // only implied
                self.pha();
            }

            Operation::PLA => {
                // only implied
                self.pla();
            }

            Operation::PHP => {
                // only implied
                self.php();
            }

            Operation::PLP => {
                // only implied
                self.plp();
            }

            //----------------------
            // INCREMENT/DECREMENT |
            //----------------------
            Operation::INX => {
                // only implied
                self.inx();
            }

            Operation::INY => {
                // only implied
                self.iny();
            }

            Operation::DEX => {
                // only implied
                self.dex();
            }

            Operation::DEY => {
                // only implied
                self.dey();
            }

            //--------
            // FLAGS |
            //--------
            Operation::CLC => {
                // only implied
                self.clc();
            }

            Operation::SEC => {
                // only implied
                self.sec();
            }

            //-------------
            // ARITHMETIC |
            //-------------
            Operation::ADC => {
                let val = self.fetch_value(instruction.mode);
                self.adc(val);
            }

            Operation::SBC => {
                let val = self.fetch_value(instruction.mode);
                self.sbc(val);
            }

            //----------
            // LOGICAL |
            //----------
            Operation::AND => {
                let val = self.fetch_value(instruction.mode);
                self.and(val);
            }

            Operation::ORA => {
                let val = self.fetch_value(instruction.mode);
                self.ora(val);
            }

            Operation::EOR => {
                let val = self.fetch_value(instruction.mode);
                self.eor(val);
            }

            //-----------------------
            // BRANCHING/COMPARISON |
            //-----------------------
            Operation::CMP => {
                let val = self.fetch_value(instruction.mode);
                self.cmp(val);
            }

            Operation::BEQ => {
                let offset = self.fetch_value(instruction.mode) as i8;
                self.beq(offset);
            }

            Operation::BNE => {
                let offset = self.fetch_value(instruction.mode) as i8;
                self.bne(offset);
            }

            _ => panic!("'{}' instruction not implemented yet", instruction.mnemonic),
        }
    }
}
