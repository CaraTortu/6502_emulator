mod operators;
mod processor;

use self::processor::Processor;

fn main() {
    // Test program for our CPU
    let test_program: Vec<u8> = vec![
        0x0a, 0x8d, 0x0b, 0x00, 0x0a, 0x0a, 0x18, 0x6d, 0x0b, 0x00, 0x60, 0x00,
    ];

    let mut processor: Processor = Processor::new();

    // Write our test program into memory
    processor.write_program(&test_program);

    // Execute
    processor.execute(0xffff);
}
