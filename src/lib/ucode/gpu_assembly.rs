pub struct GPUAssembly {}

impl GPUAssembly {
    // do nothing instruction
    pub const HLT: u8 = 0x00;
    // print stack trace
    pub const STK: u8 = 0x01;

    // bind vertex buffer
    pub const BVB: u8 = 0xa0;
    // unbind vertex buffer
    pub const UVB: u8 = 0xa1;

    // buffer vertex data
    pub const VRX: u8 = 0xab;

    // issue draw
    pub const DRW: u8 = 0xaf;
}