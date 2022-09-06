pub mod ram;
pub mod mem;

pub type Byte = u8;
pub type Word = u16;
pub type DoubleWord = u32;

impl X for Word {
    fn significant_byte(&self) -> Byte {
        todo!()
    }

    fn insignificant_byte(&self) -> Byte {
        todo!()
    }
}

trait X {
    fn significant_byte(&self) -> Byte;
    fn insignificant_byte(&self) -> Byte;
}