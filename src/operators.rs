// OPCODES for our processor
// Allow non rust approved naming for ease of reading
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub enum OPCodes {
    ORA_XIND(u8),
    ORA_YIND(u8),
    ORA_ZPG(u8),
    ORA_XZPG(u8),
    ORA_ABS(u16),
    ORA_YABS(u16),
    ORA_XABS(u16),
    ORA_IMM(u8),
    AND_XIND(u8),
    AND_YIND(u8),
    AND_ZPG(u8),
    AND_XZPG(u8),
    AND_ABS(u16),
    AND_YABS(u16),
    AND_XABS(u16),
    AND_IMM(u8),
    EOR_XIND(u8),
    EOR_YIND(u8),
    EOR_ZPG(u8),
    EOR_XZPG(u8),
    EOR_ABS(u16),
    EOR_YABS(u16),
    EOR_XABS(u16),
    EOR_IMM(u8),
    ADC_XIND(u8),
    ADC_YIND(u8),
    ADC_ZPG(u8),
    ADC_XZPG(u8),
    ADC_ABS(u16),
    ADC_YABS(u16),
    ADC_XABS(u16),
    ADC_IMM(u8),
    CMP_XIND(u8),
    CMP_YIND(u8),
    CMP_ZPG(u8),
    CMP_XZPG(u8),
    CMP_ABS(u16),
    CMP_YABS(u16),
    CMP_XABS(u16),
    CMP_IMM(u8),
    LDA_XIND(u8),
    LDA_YIND(u8),
    LDA_ZPG(u8),
    LDA_XZPG(u8),
    LDA_ABS(u16),
    LDA_YABS(u16),
    LDA_XABS(u16),
    LDA_IMM(u8),
    SBC_XIND(u8),
    SBC_YIND(u8),
    SBC_ZPG(u8),
    SBC_XZPG(u8),
    SBC_ABS(u16),
    SBC_YABS(u16),
    SBC_XABS(u16),
    SBC_IMM(u8),
    STA_XIND(u8),
    STA_YIND(u8),
    STA_ZPG(u8),
    STA_XZPG(u8),
    STA_ABS(u16),
    STA_YABS(u16),
    STA_XABS(u16),
    BMI(i8),
    BCC(i8),
    BCS(i8),
    BEQ(i8),
    BNE(i8),
    BPL(i8),
    BVC(i8),
    BVS(i8),
    ASL_ZPG(u8),
    ASL_XZPG(u8),
    ASL_A,
    ASL_ABS(u16),
    ASL_XABS(u16),
    ROL_ZPG(u8),
    ROL_XZPG(u8),
    ROL_A,
    ROL_ABS(u16),
    ROL_XABS(u16),
    LSR_ZPG(u8),
    LSR_XZPG(u8),
    LSR_A,
    LSR_ABS(u16),
    LSR_XABS(u16),
    ROR_ZPG(u8),
    ROR_XZPG(u8),
    ROR_A,
    ROR_ABS(u16),
    ROR_XABS(u16),
    LDX_ZPG(u8),
    LDX_YZPG(u8),
    LDX_ABS(u16),
    LDX_YABS(u16),
    LDX_IMM(u8),
    LDY_ZPG(u8),
    LDY_XZPG(u8),
    LDY_ABS(u16),
    LDY_XABS(u16),
    LDY_IMM(u8),
    DEC_ZPG(u8),
    DEC_XZPG(u8),
    DEC_ABS(u16),
    DEC_XABS(u16),
    INC_ZPG(u8),
    INC_XZPG(u8),
    INC_ABS(u16),
    INC_XABS(u16),
    STX_ZPG(u8),
    STX_YZPG(u8),
    STX_ABS(u16),
    STY_ZPG(u8),
    STY_XZPG(u8),
    STY_ABS(u16),
    CPX_ZPG(u8),
    CPX_ABS(u16),
    CPX_IMM(u8),
    CPY_ZPG(u8),
    CPY_ABS(u16),
    CPY_IMM(u8),
    BIT_ZPG(u8),
    BIT_ABS(u16),
    JMP_IND(u16),
    JMP_ABS(u16),
    CLC,
    CLD,
    CLI,
    CLV,
    SEC,
    SED,
    SEI,
    PHP,
    PHA,
    PLP,
    PLA,
    RTI,
    RTS,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    BRK,
    JSR(u16),
    NOP,
    DEX,
    DEY,
    INX,
    INY,
}

impl OPCodes {
    pub fn instruction_to_opcode(opcode: u8, value: Option<isize>) -> OPCodes {
        use self::OPCodes::*;

        match opcode {
            // 0
            0x00 => BRK,
            0x01 => ORA_XIND(value.unwrap() as u8),
            0x05 => ORA_ZPG(value.unwrap() as u8),
            0x06 => ASL_ZPG(value.unwrap() as u8),
            0x08 => PHP,
            0x09 => ORA_IMM(value.unwrap() as u8),
            0x0a => ASL_A,
            0x0d => ORA_ABS(value.unwrap() as u16),
            0x0e => ASL_ABS(value.unwrap() as u16),

            // 1
            0x10 => BPL(value.unwrap() as i8),
            0x11 => ORA_YIND(value.unwrap() as u8),
            0x15 => ORA_XZPG(value.unwrap() as u8),
            0x16 => ASL_XZPG(value.unwrap() as u8),
            0x18 => CLC,
            0x19 => ORA_YABS(value.unwrap() as u16),
            0x1d => ORA_XABS(value.unwrap() as u16),
            0x1e => ASL_XABS(value.unwrap() as u16),

            // 2
            0x20 => JSR(value.unwrap() as u16),
            0x21 => AND_XIND(value.unwrap() as u8),
            0x24 => BIT_ZPG(value.unwrap() as u8),
            0x25 => AND_ZPG(value.unwrap() as u8),
            0x26 => ROL_ZPG(value.unwrap() as u8),
            0x28 => PLP,
            0x29 => AND_IMM(value.unwrap() as u8),
            0x2a => ROL_A,
            0x2c => BIT_ABS(value.unwrap() as u16),
            0x2d => AND_ABS(value.unwrap() as u16),
            0x2e => ROL_ABS(value.unwrap() as u16),

            // 3
            0x30 => BMI(value.unwrap() as i8),
            0x31 => AND_YIND(value.unwrap() as u8),
            0x35 => AND_XZPG(value.unwrap() as u8),
            0x36 => ROL_XZPG(value.unwrap() as u8),
            0x38 => SEC,
            0x39 => AND_YABS(value.unwrap() as u16),
            0x3d => AND_XABS(value.unwrap() as u16),
            0x3e => ROL_XABS(value.unwrap() as u16),

            // 4
            0x40 => RTI,
            0x41 => EOR_XIND(value.unwrap() as u8),
            0x45 => EOR_ZPG(value.unwrap() as u8),
            0x46 => LSR_ZPG(value.unwrap() as u8),
            0x48 => PHA,
            0x49 => EOR_IMM(value.unwrap() as u8),
            0x4a => LSR_A,
            0x4c => JMP_ABS(value.unwrap() as u16),
            0x4d => EOR_ABS(value.unwrap() as u16),
            0x4e => LSR_ABS(value.unwrap() as u16),

            // 5
            0x51 => BVC(value.unwrap() as i8),
            0x52 => EOR_YIND(value.unwrap() as u8),
            0x55 => EOR_XZPG(value.unwrap() as u8),
            0x56 => LSR_XZPG(value.unwrap() as u8),
            0x58 => CLI,
            0x59 => EOR_YABS(value.unwrap() as u16),
            0x5d => EOR_XABS(value.unwrap() as u16),
            0x5e => LSR_XABS(value.unwrap() as u16),

            // 6
            0x60 => RTS,
            0x61 => ADC_XIND(value.unwrap() as u8),
            0x65 => ADC_ZPG(value.unwrap() as u8),
            0x66 => ROR_ZPG(value.unwrap() as u8),
            0x68 => PLA,
            0x69 => ADC_IMM(value.unwrap() as u8), // nice
            0x6a => ROR_A,
            0x6c => JMP_IND(value.unwrap() as u16),
            0x6d => ADC_ABS(value.unwrap() as u16),
            0x6e => ROR_ABS(value.unwrap() as u16),

            // 7
            0x70 => BVS(value.unwrap() as i8),
            0x71 => ADC_YIND(value.unwrap() as u8),
            0x75 => ADC_XZPG(value.unwrap() as u8),
            0x76 => ROR_XZPG(value.unwrap() as u8),
            0x78 => SEI,
            0x79 => ADC_YABS(value.unwrap() as u16),
            0x7d => ADC_XABS(value.unwrap() as u16),
            0x7e => ROR_XABS(value.unwrap() as u16),

            // 8
            0x81 => STA_XIND(value.unwrap() as u8),
            0x84 => STY_ZPG(value.unwrap() as u8),
            0x85 => STA_ZPG(value.unwrap() as u8),
            0x86 => STX_ZPG(value.unwrap() as u8),
            0x88 => DEY,
            0x8a => TXA,
            0x8c => STY_ABS(value.unwrap() as u16),
            0x8d => STA_ABS(value.unwrap() as u16),
            0x8e => STX_ABS(value.unwrap() as u16),

            // 9
            0x90 => BCC(value.unwrap() as i8),
            0x91 => STA_YIND(value.unwrap() as u8),
            0x94 => STY_XZPG(value.unwrap() as u8),
            0x95 => STA_XZPG(value.unwrap() as u8),
            0x96 => STX_YZPG(value.unwrap() as u8),
            0x98 => TYA,
            0x99 => STA_YABS(value.unwrap() as u16),
            0x9a => TXS,
            0x9d => STA_XABS(value.unwrap() as u16),

            // a
            0xa0 => LDY_IMM(value.unwrap() as u8),
            0xa1 => LDA_XIND(value.unwrap() as u8),
            0xa2 => LDX_IMM(value.unwrap() as u8),
            0xa4 => LDY_ZPG(value.unwrap() as u8),
            0xa5 => LDA_ZPG(value.unwrap() as u8),
            0xa6 => LDX_ZPG(value.unwrap() as u8),
            0xa8 => TAY,
            0xa9 => LDA_IMM(value.unwrap() as u8),
            0xaa => TAX, // Make sure to pay your taxes!
            0xac => LDY_ABS(value.unwrap() as u16),
            0xad => LDA_ABS(value.unwrap() as u16),
            0xae => LDX_ABS(value.unwrap() as u16),

            // b
            0xb0 => BCS(value.unwrap() as i8),
            0xb1 => LDA_YIND(value.unwrap() as u8),
            0xb4 => LDY_XZPG(value.unwrap() as u8),
            0xb5 => LDA_XZPG(value.unwrap() as u8),
            0xb6 => LDX_YZPG(value.unwrap() as u8),
            0xb8 => CLV,
            0xb9 => LDA_YABS(value.unwrap() as u16),
            0xba => TSX,
            0xbc => LDY_XABS(value.unwrap() as u16),
            0xbd => LDA_XABS(value.unwrap() as u16),
            0xbe => LDX_YABS(value.unwrap() as u16),

            // c
            0xc0 => CPY_IMM(value.unwrap() as u8),
            0xc1 => CMP_XIND(value.unwrap() as u8),
            0xc4 => CPY_ZPG(value.unwrap() as u8),
            0xc5 => CMP_ZPG(value.unwrap() as u8),
            0xc6 => DEC_ZPG(value.unwrap() as u8),
            0xc8 => INY,
            0xc9 => CMP_IMM(value.unwrap() as u8),
            0xca => DEX,
            0xcc => CPY_ABS(value.unwrap() as u16),
            0xcd => CMP_ABS(value.unwrap() as u16),
            0xce => DEC_ABS(value.unwrap() as u16),

            // d
            0xd0 => BNE(value.unwrap() as i8),
            0xd1 => CMP_YIND(value.unwrap() as u8),
            0xd5 => CMP_XZPG(value.unwrap() as u8),
            0xd6 => DEC_XZPG(value.unwrap() as u8),
            0xd8 => CLD,
            0xd9 => CMP_YABS(value.unwrap() as u16),
            0xdd => CMP_XABS(value.unwrap() as u16),
            0xde => DEC_XABS(value.unwrap() as u16),

            // e
            0xe0 => CPX_IMM(value.unwrap() as u8),
            0xe1 => SBC_XIND(value.unwrap() as u8),
            0xe4 => CPX_ZPG(value.unwrap() as u8),
            0xe5 => SBC_ZPG(value.unwrap() as u8),
            0xe6 => INC_ZPG(value.unwrap() as u8),
            0xe8 => INX,
            0xe9 => SBC_IMM(value.unwrap() as u8),
            0xea => NOP, // EA SPORTS
            0xec => CPX_ABS(value.unwrap() as u16),
            0xed => SBC_ABS(value.unwrap() as u16),
            0xee => INC_ABS(value.unwrap() as u16), // E

            // f
            0xf0 => BEQ(value.unwrap() as i8),
            0xf1 => SBC_YIND(value.unwrap() as u8),
            0xf5 => SBC_XZPG(value.unwrap() as u8),
            0xf6 => INC_XZPG(value.unwrap() as u8),
            0xf8 => SED,
            0xf9 => SBC_YABS(value.unwrap() as u16),
            0xfd => SBC_XABS(value.unwrap() as u16),
            0xfe => INC_XABS(value.unwrap() as u16),

            _ => panic!("ERROR: Instruction not supported: {}", opcode),
        }
    }

    // Returns the amount of bytes that it will read after the instruction
    pub fn param_count(opcode: u8) -> u16 {
        match opcode {
            // Zero length parameters
            0x00 | 0x08 | 0x0a | 0x18 | 0x28 | 0x2a | 0x38 | 0x40 | 0x48 | 0x4a | 0x58 | 0x60
            | 0x68 | 0x6a | 0x78 | 0x88 | 0x8a | 0x98 | 0x9a | 0xa8 | 0xaa | 0xb8 | 0xba | 0xc8
            | 0xca | 0xd8 | 0xe8 | 0xea | 0xf8 => 0,

            // Byte parameters
            0x01..=0x06
            | 0x09
            | 0x10..=0x16
            | 0x21..=0x26
            | 0x29
            | 0x30..=0x36
            | 0x41..=0x46
            | 0x49
            | 0x50..=0x56
            | 0x61..=0x66
            | 0x69
            | 0x70..=0x76
            | 0x81..=0x86
            | 0x90..=0x96
            | 0xa0..=0xa6
            | 0xa9
            | 0xb0..=0xb6
            | 0xc0..=0xc6
            | 0xc9
            | 0xd0..=0xd6
            | 0xe0..=0xe6
            | 0xe9
            | 0xf0..=0xf6 => 1,

            // Word parameters
            _ => 2,
        }
    }
}
