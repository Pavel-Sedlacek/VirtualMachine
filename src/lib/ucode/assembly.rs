pub struct Assembly {

}

impl Assembly {

    // do nothing instruction
    pub const HLT: u8 = 0x00;
    pub const STK: u8 = 0x01;

    // load a
    pub const LDA: u8 = 0x40;
    // load x
    pub const LDX: u8 = 0x41;
    // load y
    pub const LDY: u8 = 0x42;

    // transfer a to x
    pub const TAX: u8 = 0x50;
    // transfer a to y
    pub const TAY: u8 = 0x51;
    // transfer x to a
    pub const TXA: u8 = 0x52;
    // transfer x to y
    pub const TXY: u8 = 0x53;
    // transfer y to a
    pub const TYA: u8 = 0x54;
    // transfer y to x
    pub const TYX: u8 = 0x55;

    // store a to memory
    pub const STA: u8 = 0x60;
    // store x to memory
    pub const STX: u8 = 0x61;
    // store y to memory
    pub const STY: u8 = 0x62;

    // push a to stack
    pub const PSA: u8 = 0x70;
    // push x to stack
    pub const PSX: u8 = 0x71;
    // push y to stack
    pub const PSY: u8 = 0x72;

    // push program counter to stack
    pub const PSP: u8 = 0x73;

    // pull a from stack
    pub const PLA: u8 = 0x76;
    // pull x from stack
    pub const PLX: u8 = 0x77;
    // pull y from stack
    pub const PLY: u8 = 0x78;

    // pull program counter from stack
    pub const PLP: u8 = 0x79;

    // compare to a
    pub const CMP: u8 = 0xa0;
    // compare to x
    pub const CMX: u8 = 0xa1;
    // compare to y
    pub const CMY: u8 = 0xa2;
    // compare a to x
    pub const CAX: u8 = 0xa3;
    // compare a to y
    pub const CAY: u8 = 0xa4;
    // compare x to y
    pub const CXY: u8 = 0xa5;

    // branch zero flag
    pub const BEQ: u8 = 0xaa;
    // branch non zero flag
    pub const BNE: u8 = 0xab;
    // jump
    pub const JMP: u8 = 0xac;

    // dec a
    pub const DEC: u8 = 0xb0;
    // dec x
    pub const DEX: u8 = 0xb1;
    // dec y
    pub const DEY: u8 = 0xb2;

    // inc a
    pub const INC: u8 = 0xba;
    // inc x
    pub const INX: u8 = 0xbb;
    // inc y
    pub const INY: u8 = 0xbc;
}