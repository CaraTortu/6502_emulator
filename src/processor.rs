use crate::operators::OPCodes;

// Processor based on the 6502
// Components:
//   - Stack (0x0000 -> 0xffff)
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
pub struct Processor {
    stack: [u8; 0xffff],
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
            stack: [0; 0xffff],
            a: 0,
            x: 0,
            y: 0,
            pc: 0x0000, // Hardcode program counter to 0x0000
            sp: 0x0000, // Hardcode Stack pointer to 0x0200
            sr: 0b11111111,
        }
    }

    // STACK OPERATIONS
    pub fn read_byte(&self, address: u16) -> Option<u8> {
        Some(self.stack.get(address as usize)?.to_owned())
    }

    pub fn write_byte(&mut self, address: u16, data: u8) {
        self.stack[address as usize] = data.to_le_bytes()[0];
    }

    pub fn read_word(&self, address: u16) -> Option<u16> {
        Some((self.read_byte(address + 1)? as u16) << 8 | self.read_byte(address)? as u16)
    }

    pub fn write_word(&mut self, address: u16, data: u16) {
        data.to_le_bytes()
            .iter()
            .enumerate()
            .map(|(i, d)| self.write_byte(address + i as u16, *d))
            .count();
    }

    pub fn write_program(&mut self, program: &Vec<u8>) {
        self.sp = program.len() as u16 + 1;
        for (address, byte) in program.iter().enumerate() {
            self.write_byte(address as u16, byte.to_owned());
        }
    }

    // OPCODES handling
    pub fn execute(&mut self, cycle_limit: u64) {
        let mut cycles: u64 = 0;

        while cycles < cycle_limit {
            let instruction = self.read_byte(self.pc).unwrap();
            let difference: u16;

            let parameter: Option<isize> = match OPCodes::param_count(instruction) {
                0 => {
                    difference = 1;
                    None
                }
                1 => {
                    difference = 2;
                    Some(self.read_byte(self.pc + 1).unwrap() as isize)
                }
                _ => {
                    difference = 3;
                    Some(self.read_word(self.pc + 1).unwrap() as isize)
                }
            };

            // Increase the program counter by the amount of bytes read
            self.pc += difference;

            // Get the OPcode related to the hex code
            let opcode = OPCodes::instruction_to_opcode(instruction, parameter);

            // Increase the amount of cycles we have gone through
            cycles += self.handle_opcode(&opcode);

            // Return of the program means we finished. Break out of loop.
            if opcode == OPCodes::RTS {
                break;
            }
        }
    }

    pub fn handle_opcode(&mut self, instruction: &OPCodes) -> u64 {
        //use OPCodes::*;

        println!("{:?}", instruction);
        return 1;
    }
}

mod test {
    #[allow(unused)]
    use super::*;

    #[test]
    pub fn write_and_write_byte() {
        let mut processor = Processor::new();

        processor.write_byte(0xff2f, 0x33);
        assert_eq!(processor.read_byte(0xff2f), Some(0x33));
    }

    #[test]
    pub fn write_and_read_word() {
        let mut processor = Processor::new();

        processor.write_word(0xff2f, 0xf6e4);
        assert_eq!(processor.read_word(0xff2f), Some(0xf6e4));
    }
}
