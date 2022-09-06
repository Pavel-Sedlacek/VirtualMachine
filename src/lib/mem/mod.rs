pub mod ram;
pub mod mem;

pub type Byte = u8;
pub type Word = u16;
pub type DoubleWord = u32;

impl X for Word {
    fn significant_byte(&self) -> Byte {
        (self >> 8) as Byte
    }

    fn insignificant_byte(&self) -> Byte {
        *self as Byte
    }
}

trait X {
    fn significant_byte(&self) -> Byte;
    fn insignificant_byte(&self) -> Byte;
}