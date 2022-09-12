use crate::lib::mem::Word;

pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Color {
    pub fn as_word(&self) -> Word {
        return 0x11001111;
    }
}