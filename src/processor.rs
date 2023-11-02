use std::io::Error;

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
struct Processor {
    stack: [u8; 0xffff],
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u16,
    sr: u8
}

impl Processor {
    // Create a new 6502 Processor
    pub fn new() -> Self {
        Self {
            stack: [0; 0xffff],
            a: 0,
            x: 0,
            y: 0,
            pc: 0xfce2, // Hardcode program counter to 0xfec2
            sp: 0x01fd, // Hardcode Stack pointer to 0x01fd
            sr: 0b11111111
        }
    }

    // STACK OPERATIONS
    pub fn read_byte(&self, address: u16) -> Option<&u8> {
        self.stack.get(address as usize)
    }

    pub fn write_byte(&mut self, address: u16, data: u8) {
        self.stack[address as usize] = data;
    }

    

}