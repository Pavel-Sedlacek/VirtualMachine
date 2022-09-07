pub mod ram;
pub mod mem;

pub type Byte = u8;
pub type Word = u16;
pub type DoubleWord = u32;

impl B for Byte {
    fn set_bit(&self, index: usize) -> Byte {
        self | 1 << index
    }

    fn unset_bit(&self, index: usize) -> Byte {
        self & !(1 << index)
    }

    fn is_set_bit(&self, index: usize) -> bool {
        ((1 << index) & self) != 0
    }
}

impl W for Word {
    fn significant_byte(&self) -> Byte {
        (self >> 8) as Byte
    }

    fn insignificant_byte(&self) -> Byte {
        *self as Byte
    }
}

impl D for DoubleWord {
    fn significant_word(&self) -> Word {
        (self >> 16) as Word
    }

    fn insignificant_word(&self) -> Word {
        *self as Word
    }
}


pub trait B {
    fn set_bit(&self, index: usize) -> Byte;
    fn unset_bit(&self, index: usize) -> Byte;
    fn is_set_bit(&self, index: usize) -> bool;
}

pub trait W {
    fn significant_byte(&self) -> Byte;
    fn insignificant_byte(&self) -> Byte;
}

pub trait D {
    fn significant_word(&self) -> Word;
    fn insignificant_word(&self) -> Word;
}