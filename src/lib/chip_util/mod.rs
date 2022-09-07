use crate::lib::mem::{Byte, DoubleWord, Word};

pub fn combine_to_word(significant_byte: Byte, insignificant_byte: Byte) -> Word {
    ((significant_byte as u16) << 8) | (insignificant_byte) as Word
}
pub fn combine_to_double_word(significant_word: Word, insignificant_word: Word) -> DoubleWord {
    ((significant_word as u32) << 16) | (insignificant_word) as DoubleWord
}