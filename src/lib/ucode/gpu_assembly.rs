pub struct GPUAssembly {}

impl GPUAssembly {
    // do nothing instruction
    pub const HLT: u8 = 0x00;
    // print stack trace
    pub const STK: u8 = 0x01;


    pub const VAO: u8 = 0x01;

    // buffer vertex data
    // 0bXXXXXXXX_YYYYYYYY_CCCCCCCC_AABBCCDD
    // xx = x position u8
    // yy = y position u8
    // cc = color u8 [rrggbbaa]
    // aa = texture coordinate {00; 01; 10; 11}
    // bb = z-index 0b00 - 0b11
    pub const VRX: u8 = 0x20;

    // connect vertices to face
    pub const FCE: u8 = 0x21;

    // bind face texture

}