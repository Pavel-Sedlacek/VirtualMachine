use crate::lib::mem::Byte;

pub struct UCode {

}

impl UCode {
    
    // cpu uCode
    pub const GENERIC_CPU_FAILURE: Byte = 0xa0;
    pub const REGISTER_OVERFLOW_FAILURE: Byte = 0xa1;
    pub const POINTER_UNDERFLOW_FAILURE: Byte = 0xa2;

    // gpu uCode
    pub const MONITOR_NOT_FOUND: Byte = 0xb0;
    pub const PIXEL_OUT_OF_BOUNDS: Byte = 0xb1;

    // memory uCode
    pub const GENERIC_MEMORY_FAILURE: Byte = 0xd0;
    pub const INVALID_MEMORY_READ: Byte = 0xd1;
    pub const INVALID_MEMORY_WRITE: Byte = 0xd2;
    pub const MEMORY_ALREADY_LOCKED: Byte = 0xd3;
    pub const MEMORY_ALREADY_UNLOCKED: Byte = 0xd4;

    // Buffer uCode
    pub const INVALID_BUFFER_ACCESS: Byte = 0xe0;

    //
    pub const UNKNOWN_EXCEPTION: Byte = 0xfe;
    // generic uCode
    pub const HLT: Byte = 0xff;
}