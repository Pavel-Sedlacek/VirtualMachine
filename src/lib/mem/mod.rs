use std::intrinsics::powf32;

pub mod ram;
pub mod mem;

pub type Byte = u8;
pub type Word = u16;
pub type DoubleWord = u32;

impl W for Word {
    fn significant_byte(&self) -> Byte {
        (self >> 8) as Byte
    }

    fn insignificant_byte(&self) -> Byte {
        *self as Byte
    }
}

impl B for Byte {
    fn set_bit(&self, index: usize) -> Byte {
        self | 1 << index
    }

    fn unset_bit(&self, index: usize) -> Byte {
        self & !(1 << index)
    }
}

pub trait W {
    fn significant_byte(&self) -> Byte;
    fn insignificant_byte(&self) -> Byte;
}
pub trait B {
    fn set_bit(&self, index: usize) -> Byte;
    fn unset_bit(&self, index: usize) -> Byte;
}