use crate::mem::Byte;

pub struct UCode {

}

impl UCode {

    pub const GENERIC_CPU_FAILURE: Byte = 0xa0;
    pub const REGISTER_OVERFLOW_FAILURE: Byte = 0xa1;
    pub const POINTER_UNDERFLOW_FAILURE: Byte = 0xa2;

    pub const GENERIC_MEMORY_FAILURE: Byte = 0xd0;
    pub const INVALID_MEMORY_READ: Byte = 0xd1;
    pub const INVALID_MEMORY_WRITE: Byte = 0xd2;
    pub const MEMORY_ALREADY_LOCKED: Byte = 0xd3;
    pub const MEMORY_ALREADY_UNLOCKED: Byte = 0xd4;



    pub const TERMINATE: Byte = 0xfd;
    pub const RETRY: Byte = 0xfe;
    pub const CONTINUE: Byte = 0xff;
}