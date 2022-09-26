use crate::lib::mem::Word;

pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Color {
    pub fn as_word(&self) -> Word {
        return 0b11001111;
    }

    pub fn white() -> Self {
        Color {
            red: u8::MAX,
            green: u8::MAX,
            blue: u8::MAX,
            alpha: u8::MAX,
        }
    }
    pub fn black() -> Self {
        Color {
            red: u8::MIN,
            green: u8::MIN,
            blue: u8::MIN,
            alpha: u8::MAX,
        }
    }

    pub fn r(&self) -> u8 {
        self.red
    }
    pub fn g(&self) -> u8 {
        self.green
    }
    pub fn b(&self) -> u8 {
        self.blue
    }
}

impl Clone for Color {
    fn clone(&self) -> Self {
        Color {
            red: self.red,
            green: self.green,
            blue: self.blue,
            alpha: self.alpha
        }
    }
}