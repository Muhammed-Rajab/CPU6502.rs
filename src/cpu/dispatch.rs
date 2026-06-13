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

    // LDX
    table[0xA2] = instr("LDX", Operation::LDX, AddressingMode::Immediate, 2, 2);

    // LDY
    table[0xA0] = instr("LDY", Operation::LDY, AddressingMode::Immediate, 2, 2);

    //--------
    // STORE |
    //--------

    // STA
    table[0x8D] = instr("STA", Operation::STA, AddressingMode::Absolute, 3, 4);
    table[0x85] = instr("STA", Operation::STA, AddressingMode::ZeroPage, 2, 3);

    // STX
    table[0x8E] = instr("STX", Operation::STX, AddressingMode::Absolute, 3, 4);
    table[0x86] = instr("STX", Operation::STX, AddressingMode::ZeroPage, 2, 3);

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

            _ => panic!("'{}' instruction not implemented yet", instruction.mnemonic),
        }
    }

    // pub fn step(&mut self) {
    //     let opcode = self.fetch_byte();
    //
    //     println!("PC=${:04X}, OPCODE=${:02X}", self.pc, opcode);
    //
    //     match opcode {
    //         //------
    //         // LDA |
    //         //------
    //
    //         // Immediate
    //         0xA9 => {
    //             let value = self.fetch_byte();
    //             self.lda(value);
    //         }
    //
    //         //------
    //         // LDX |
    //         //------
    //
    //         // Immediate
    //         0xA2 => {
    //             let value = self.fetch_byte();
    //             self.ldx(value);
    //         }
    //
    //         //------
    //         // LDY |
    //         //------
    //
    //         // Immediate
    //         0xA0 => {
    //             let value = self.fetch_byte();
    //             self.ldy(value);
    //         }
    //
    //         //------
    //         // STA |
    //         //------
    //
    //         // Absolute
    //         0x8D => {
    //             // $1234 in memory -> $34 $12
    //             let low = self.fetch_byte() as u16;
    //             let high = self.fetch_byte() as u16;
    //             let addr = (high << 8) | low;
    //             self.sta(addr);
    //         }
    //
    //         // Zero Page
    //         0x85 => {
    //             // $00-$ff
    //             let addr = self.fetch_byte() as u16;
    //             self.sta(addr);
    //         }
    //
    //         //------
    //         // STX |
    //         //------
    //
    //         // Absolute
    //         0x8E => {
    //             // $1234 in memory -> $34 $12
    //             let low = self.fetch_byte() as u16;
    //             let high = self.fetch_byte() as u16;
    //             let addr = (high << 8) | low;
    //             self.stx(addr);
    //         }
    //
    //         // Zero Page
    //         0x86 => {
    //             // $00-$ff
    //             let addr = self.fetch_byte() as u16;
    //             self.stx(addr);
    //         }
    //
    //         //------
    //         // STY |
    //         //------
    //
    //         // Absolute
    //         0x8C => {
    //             // $1234 in memory -> $34 $12
    //             let low = self.fetch_byte() as u16;
    //             let high = self.fetch_byte() as u16;
    //             let addr = (high << 8) | low;
    //             self.sty(addr);
    //         }
    //
    //         // Zero Page
    //         0x84 => {
    //             // $00-$ff
    //             let addr = self.fetch_byte() as u16;
    //             self.sty(addr);
    //         }
    //
    //         //------
    //         // TAX |
    //         //------
    //
    //         // Implied
    //         0xAA => {
    //             self.tax();
    //         }
    //
    //         //------
    //         // TAY |
    //         //------
    //
    //         // Implied
    //         0xA8 => {
    //             self.tay();
    //         }
    //
    //         //------
    //         // TXA |
    //         //------
    //
    //         // Implied
    //         0x8A => {
    //             self.txa();
    //         }
    //
    //         //------
    //         // TYA |
    //         //------
    //
    //         // Implied
    //         0x98 => {
    //             self.tya();
    //         }
    //
    //         //------
    //         // TSX |
    //         //------
    //
    //         // Implied
    //         0xBA => {
    //             self.tsx();
    //         }
    //
    //         //------
    //         // TXS |
    //         //------
    //
    //         // Implied
    //         0x9A => {
    //             self.txs();
    //         }
    //
    //         //------
    //         // PHA |
    //         //------
    //
    //         // Implied
    //         0x48 => {
    //             self.pha();
    //         }
    //
    //         //------
    //         // PLA |
    //         //------
    //
    //         // Implied
    //         0x68 => {
    //             self.pla();
    //         }
    //
    //         //------
    //         // PHP |
    //         //------
    //
    //         // Implied
    //         0x08 => {
    //             self.php();
    //         }
    //
    //         //------
    //         // PLP |
    //         //------
    //
    //         // Implied
    //         0x28 => {
    //             self.plp();
    //         }
    //
    //         //------
    //         // INX |
    //         //------
    //
    //         // Implied
    //         0xE8 => {
    //             self.inx();
    //         }
    //
    //         //------
    //         // INY |
    //         //------
    //
    //         // Implied
    //         0xC8 => {
    //             self.iny();
    //         }
    //
    //         //------
    //         // DEX |
    //         //------
    //
    //         // Implied
    //         0xCA => {
    //             self.dex();
    //         }
    //
    //         //------
    //         // DEY |
    //         //------
    //
    //         // Implied
    //         0x88 => {
    //             self.dey();
    //         }
    //
    //         //------
    //         // CLC |
    //         //------
    //
    //         // Implied
    //         0x18 => {
    //             self.clc();
    //         }
    //
    //         //------
    //         // SEC |
    //         //------
    //
    //         // Implied
    //         0x38 => {
    //             self.sec();
    //         }
    //
    //         //------
    //         // ADC |
    //         //------
    //
    //         // Immediate
    //         0x69 => {
    //             let val = self.fetch_byte();
    //             self.adc(val);
    //         }
    //
    //         //------
    //         // SBC |
    //         //------
    //
    //         // Immediate
    //         0xE9 => {
    //             let val = self.fetch_byte();
    //             self.sbc(val);
    //         }
    //
    //         //------
    //         // AND |
    //         //------
    //
    //         // Immediate
    //         0x29 => {
    //             let val = self.fetch_byte();
    //             self.and(val);
    //         }
    //
    //         //------
    //         // ORA |
    //         //------
    //
    //         // Immediate
    //         0x09 => {
    //             let val = self.fetch_byte();
    //             self.ora(val);
    //         }
    //
    //         //------
    //         // EOR |
    //         //------
    //
    //         // Immediate
    //         0x49 => {
    //             let val = self.fetch_byte();
    //             self.eor(val);
    //         }
    //
    //         //------
    //         // CMP |
    //         //------
    //
    //         // Immediate
    //         0xC9 => {
    //             let val = self.fetch_byte();
    //             self.cmp(val);
    //         }
    //
    //         //------
    //         // BEQ |
    //         //------
    //
    //         // Relative
    //         0xF0 => {
    //             let val = self.fetch_byte();
    //             self.beq(val);
    //         }
    //
    //         //------
    //         // BNE |
    //         //------
    //
    //         // Relative
    //         0xD0 => {
    //             let val = self.fetch_byte();
    //             self.bne(val);
    //         }
    //
    //         // Default
    //         _ => panic!("unknown opcode: {:02x}", opcode),
    //     };
    // }
}
