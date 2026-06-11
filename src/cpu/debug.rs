//-----------------------------------------------
// DEBUG                                        |
//-----------------------------------------------

use super::Cpu6502;

impl Cpu6502 {
    pub fn hexdump(&self, start: u16, len: u16) {
        /*
         * given,
         *      start = 0x1000
         *      len   = 10
         * the function outputs values from 0x1000..0x1009
         * [start, end)
         *
         * TODO: ASCII output
         */
        let aligned_start = start & !0x000F;
        let end = start
            .checked_add(len)
            .expect("hexdump range exceeds address space");
        let aligned_end = (end + 0x000F) & !0x000F;

        // print top row showing byte alignment
        print!("      ");
        for i in 0..=15 {
            print!("{:02x} ", i);
        }
        println!();

        // dump memory
        for addr in aligned_start..aligned_end {
            // print start if addr % 16 == 0
            if (addr & 0x000F) == 0 {
                print!("\n");
                print!("${:04x} ", addr);
            }

            // print '-- ' if addr < start
            if addr < start || addr >= end {
                print!("-- ");
            } else {
                // read and print the value if addr in [start, end)
                let byte = self.read(addr);
                print!("{:02x} ", byte);
            }
        }

        println!();
    }
}
