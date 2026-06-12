//-----------------------------------------------
// OPERATION                                    |
//-----------------------------------------------

use super::Cpu6502;

impl Cpu6502 {
    pub fn step(&mut self) {
        let opcode = self.fetch_byte();

        println!("PC=${:04X}, OPCODE=${:02X}", self.pc, opcode);

        match opcode {
            //------
            // LDA |
            //------

            // Immediate
            0xA9 => {
                let value = self.fetch_byte();
                self.lda(value);
            }

            //------
            // LDX |
            //------

            // Immediate
            0xA2 => {
                let value = self.fetch_byte();
                self.ldx(value);
            }

            //------
            // LDY |
            //------

            // Immediate
            0xA0 => {
                let value = self.fetch_byte();
                self.ldy(value);
            }

            //------
            // STA |
            //------

            // Absolute
            0x8D => {
                // $1234 in memory -> $34 $12
                let low = self.fetch_byte() as u16;
                let high = self.fetch_byte() as u16;
                let addr = (high << 8) | low;
                self.sta(addr);
            }

            // Zero Page
            0x85 => {
                // $00-$ff
                let addr = self.fetch_byte() as u16;
                self.sta(addr);
            }

            //------
            // STX |
            //------

            // Absolute
            0x8E => {
                // $1234 in memory -> $34 $12
                let low = self.fetch_byte() as u16;
                let high = self.fetch_byte() as u16;
                let addr = (high << 8) | low;
                self.stx(addr);
            }

            // Zero Page
            0x86 => {
                // $00-$ff
                let addr = self.fetch_byte() as u16;
                self.stx(addr);
            }

            //------
            // STY |
            //------

            // Absolute
            0x8C => {
                // $1234 in memory -> $34 $12
                let low = self.fetch_byte() as u16;
                let high = self.fetch_byte() as u16;
                let addr = (high << 8) | low;
                self.sty(addr);
            }

            // Zero Page
            0x84 => {
                // $00-$ff
                let addr = self.fetch_byte() as u16;
                self.sty(addr);
            }

            //------
            // TAX |
            //------

            // Implied
            0xAA => {
                self.tax();
            }

            //------
            // TAY |
            //------

            // Implied
            0xA8 => {
                self.tay();
            }

            //------
            // TXA |
            //------

            // Implied
            0x8A => {
                self.txa();
            }

            //------
            // TYA |
            //------

            // Implied
            0x98 => {
                self.tya();
            }

            //------
            // TSX |
            //------

            // Implied
            0xBA => {
                self.tsx();
            }

            //------
            // TXS |
            //------

            // Implied
            0x9A => {
                self.txs();
            }

            //------
            // PHA |
            //------

            // Implied
            0x48 => {
                self.pha();
            }

            //------
            // PLA |
            //------

            // Implied
            0x68 => {
                self.pla();
            }

            //------
            // PHP |
            //------

            // Implied
            0x08 => {
                self.php();
            }

            //------
            // PLP |
            //------

            // Implied
            0x28 => {
                self.plp();
            }

            //------
            // INX |
            //------

            // Implied
            0xE8 => {
                self.inx();
            }

            //------
            // INY |
            //------

            // Implied
            0xC8 => {
                self.iny();
            }

            //------
            // DEX |
            //------

            // Implied
            0xCA => {
                self.dex();
            }

            //------
            // DEY |
            //------

            // Implied
            0x88 => {
                self.dey();
            }

            //------
            // CLC |
            //------

            // Implied
            0x18 => {
                self.clc();
            }

            //------
            // SEC |
            //------

            // Implied
            0x38 => {
                self.sec();
            }

            //------
            // ADC |
            //------

            // Immediate
            0x69 => {
                let val = self.fetch_byte();
                self.adc(val);
            }

            //------
            // SBC |
            //------

            // Immediate
            0xE9 => {
                let val = self.fetch_byte();
                self.sbc(val);
            }

            // Default
            _ => panic!("unknown opcode: {:02x}", opcode),
        };
    }
}
