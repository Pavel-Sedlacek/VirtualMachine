pub struct GPUAssembly {}

impl GPUAssembly {
    // do nothing instruction
    pub const HLT: u8 = 0x00;

    // print stack trace
    pub const STK: u8 = 0x01;
}