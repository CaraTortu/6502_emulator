mod processor;
mod operators;

use self::operators::OPCodes;
use OPCodes::*;

fn main() {
    // Test program for our CPU
    let test_program: [u8; 40] = [0xa0, 0x00, 0x84, 0x32, 0xb1, 0x30, 0xaa, 0xc8, 0xca, 0xb1, 0x30, 0xc8, 0xd1, 
                                  0x30, 0x90, 0x10, 0xf0, 0x0e, 0x48, 0xb1, 0x30, 0x88, 0x91, 0x30, 0x68, 0xc8, 
                                  0x91, 0x30, 0xa9, 0xff, 0x85, 0x32, 0xca, 0xd0, 0xe6, 0x24, 0x32, 0x30, 0xd9, 0x60];


    // Translate hex to enum variants
    let mut initial_program: Vec<OPCodes> = vec![];
    let mut byte: usize = 0;

    while byte < test_program.len() - 1 {
        // Check whether it has parameters or not and add accordingly
        let param: Option<isize> = match OPCodes::instruction_to_opcode(test_program[byte], Some(test_program[byte+1] as isize)) {
            CLC | CLD | CLI | CLV | SEC | 
            SED | SEI | PHP | PHA | PLP | 
            PLA | RTI | RTS | TAX | TAY | 
            TSX | TXA | TXS | TYA | DEX | 
            DEY | INX | INY | BRK | NOP => {
                initial_program.push(OPCodes::instruction_to_opcode(test_program[byte], None));
                None
            },
            _ => {
                let ret_val = Some(test_program[byte+1] as isize);
                initial_program.push(OPCodes::instruction_to_opcode(test_program[byte], ret_val));
                ret_val
            },
        };
       
        if let Some(_) = param {
            byte += 2;
        } else {
            byte += 1;
        }
    }

    println!("{:?}", initial_program);
}
