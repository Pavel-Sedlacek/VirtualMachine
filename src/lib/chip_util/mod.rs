use crate::lib::mem::{Byte, Word};

pub fn combine_to_word(significant_byte: Byte, insignificant_byte: Byte) -> Word {
    ((significant_byte as u16) << 8) | (insignificant_byte) as Word
}