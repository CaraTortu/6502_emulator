use crate::operators::OPCodes;

// Processor based on the 6502
// Components:
//   - RAM:
//      - Zero page (0x0000 -> 0x00ff)
//      - Stack (0x0100 -> 0x01ff)
//      - Program (0x200 -> 0xfffb)
//      - EXEC START -> 0xfffc+0xfffd
//   - Registers:
//      - Accumulator (A): 8 bits
//      - X: 8 bits
//      - Y: 8 bits
//      - Program Counter (PC): 16 bits
//      - Stack Pointer (SP): 8 bits
//      - Status (SR) (Bits meaning): 8bits
//          - 7th: Negative
//          - 6th: Overflow
//          - 5th: -
//          - 4th: Break
//          - 3rd: Decimal
//          - 2nd: Interrupt disable
//          - 1st: Zero
//          - 0th: Carry
#[allow(dead_code)]
pub struct Processor {
    ram: [u8; 0xffff],
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u16,
    sr: u8,
}

impl Processor {
    // Create a new 6502 Processor
    pub fn new() -> Self {
        Self {
            ram: [0; 0xffff],
            a: 0,
            x: 0,
            y: 0,

            // Hardcode program counter to 0xFFFC.
            // It will read a word from the address and JMP to that address.
            pc: 0xFFFC,

            // Hardcode Stack pointer to 0x0100
            // (Stack address space is from 0x0100 to 0x01ff.
            sp: 0x0100,
            sr: 0b11111111,
        }
    }

    // STACK OPERATIONS
    pub fn read_byte(&mut self) -> Option<u8> {
        self.pc += 1;
        Some(self.ram.get(self.pc as usize - 1)?.to_owned())
    }

    pub fn read_byte_at_address(&self, address: u16) -> Option<u8> {
        Some(self.ram.get(address as usize)?.to_owned())
    }

    pub fn write_byte(&mut self, address: u16, data: u8) {
        self.ram[address as usize] = data.to_le_bytes()[0];
    }

    pub fn read_word(&mut self) -> Option<u16> {
        Some(self.read_byte()? as u16 | (self.read_byte()? as u16) << 8)
    }

    pub fn read_word_at_address(&mut self, address: u16) -> Option<u16> {
        Some(
            self.read_byte_at_address(address)? as u16
                | (self.read_byte_at_address(address + 1)? as u16) << 8,
        )
    }

    pub fn write_word(&mut self, address: u16, data: u16) {
        data.to_le_bytes()
            .iter()
            .enumerate()
            .map(|(i, d)| self.write_byte(address + i as u16, *d))
            .count();
    }

    pub fn write_program(&mut self, program: &Vec<u8>) {
        // Write the program starting at 0x0200
        for (address, byte) in program.iter().enumerate() {
            // Make sure we are NOT writing past 0xfffb
            assert!(address as u16 + 0x0200 < 0xfffc);
            self.write_byte(address as u16 + 0x0200, byte.to_owned());
        }

        // Write the start address to start executing from
        self.write_word(0xfffc, 0x0200);

        // Set the program counter to 0x0200 for now until we can program the JMP instruction
        self.pc = 0x0200;
    }

    // OPCODES handling
    pub fn execute(&mut self, cycle_limit: u64) {
        let mut cycles: u64 = 0;

        while cycles < cycle_limit {
            let instruction = self.read_byte().unwrap();
            let param_count = OPCodes::param_count(instruction);

            let parameter: Option<isize> = match param_count {
                0 => None,
                1 => Some(self.read_byte().unwrap() as isize),
                _ => Some(self.read_word().unwrap() as isize),
            };

            // Get the OPcode related to the hex code
            let opcode = OPCodes::instruction_to_opcode(instruction, parameter);

            // Return of the program means we finished. Break out of loop.
            if opcode == OPCodes::RTS {
                break;
            }

            // Increase the amount of cycles we have gone through
            cycles += self.handle_opcode(opcode);
        }
    }

    pub fn handle_opcode(&mut self, instruction: OPCodes) -> u64 {
        use OPCodes::*;

        match instruction {
            // Set the CARRY flag
            SEC => self.sr |= 0b0000_0001,
            // Set the INTERRUPT DISABLE flag
            SEI => self.sr |= 0b0000_0100,
            // Set DECIMAL MODE flag
            SED => self.sr |= 0b0000_1000,
            // Clear the CARRY flag
            CLC => self.sr &= 0b1111_1110,
            // Clear the OVERFLOW flag
            CLV => self.sr &= 0b1011_1111,
            // Clear the INTERRUPT DISABLE flag
            CLI => self.sr &= 0b1111_1011,
            // Clear the DECIMAL MODE flag
            CLD => self.sr &= 0b1111_0111,

            // Handle LDA cases. Will later be unabstracted for the sake of performance. Currently
            // abstracted for debug purposes.
            LDA_ABS(_) | LDA_ZPG(_) | LDA_XABS(_) | LDA_XIND(_) | LDA_XZPG(_) | LDA_YABS(_)
            | LDA_YIND(_) | LDA_IMM(_) => self.handle_lda(instruction),

            // Other shit todo
            _ => (),
        };

        return 1;
    }

    fn handle_lda(&mut self, instruction: OPCodes) {
        use OPCodes::*;

        match instruction {
            LDA_IMM(value) => self.a = value,

            // Handle absolute values.
            LDA_ABS(value) => self.a = self.read_byte_at_address(value).unwrap(),
            LDA_XABS(value) => self.a = self.read_byte_at_address(value + self.x as u16).unwrap(),
            LDA_YABS(value) => self.a = self.read_byte_at_address(value + self.y as u16).unwrap(),

            // The two lines below are operated by 0xff.
            // This is to prevent the address addition to go past the zero page address since we
            // only apply the sum to the LSB.
            LDA_ZPG(value) => self.a = self.read_byte_at_address(value as u16 % 0xff).unwrap(),
            LDA_XZPG(value) => {
                self.a = self
                    .read_byte_at_address((value + self.x) as u16 % 0xff)
                    .unwrap()
            }

            // Handle indirect values.
            LDA_XIND(value) => {
                let address: u16 = self
                    .read_word_at_address(value as u16 + self.x as u16)
                    .unwrap();
                self.a = self.read_byte_at_address(address).unwrap()
            }
            LDA_YIND(value) => {
                let address: u16 = self.read_word_at_address(value as u16).unwrap();
                self.a = self.read_byte_at_address(address + self.y as u16).unwrap()
            }

            // If we got here is because the universe made a bit flip an magically changed the
            // value at this specific time to some other complete nonesense. Oddly specific?
            _ => unreachable!(),
        }
    }
}

mod test {
    #[allow(unused)]
    use super::*;

    #[test]
    pub fn write_and_write_byte() {
        let mut processor = Processor::new();
        processor.pc = 0xff2f;
        processor.write_byte(0xff2f, 0x33);
        assert_eq!(processor.read_byte(), Some(0x33));
    }

    #[test]
    pub fn write_and_read_word() {
        let mut processor = Processor::new();

        processor.pc = 0xff2f;
        processor.write_word(0xff2f, 0xf6e4);
        assert_eq!(processor.read_word(), Some(0xf6e4));
    }

    #[test]
    pub fn test_lda() {
        let mut processor = Processor::new();

        // Write the value 0x30 into ff2f
        processor.write_byte(0xff2f, 0x30);

        // Write the program
        processor.write_program(&vec![0xad, 0x2f, 0xff, 0x60]);
        
        // Execute
        const MAX_CYCLES: u64 = 0xffff;
        processor.execute(MAX_CYCLES);

        // Check A
        assert_eq!(processor.a, 0x30);
    }
}
