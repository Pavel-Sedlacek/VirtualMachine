use crate::lib::chip_util::{combine_to_double_word, combine_to_word};
use crate::lib::mem::{Byte, DoubleWord, W, Word};
use crate::lib::ucode::ucode::UCode;

pub struct RAM {
    size: usize,
    memory: Vec<Byte>,
    lock: bool,
}

impl RAM {
    pub fn new(size: usize) -> Self {
        RAM {
            size,
            memory: vec![0x0; size],
            lock: false
        }
    }
}

impl RAM {
    pub fn lock(&mut self) -> Result<(), Byte> {
        if self.lock {Err(UCode::MEMORY_ALREADY_LOCKED)}
        else {
            self.lock = true;
            Ok(())
        }
    }
    pub fn unlock(&mut self) -> Result<(), Byte> {
        if self.lock {
            self.lock = false;
            Ok(())
        }
        else {
            Err(UCode::MEMORY_ALREADY_UNLOCKED)
        }
    }
    pub fn is_locked(&self) -> bool {
        self.lock
    }

    pub fn fetch_byte(&self, address: usize) -> Result<Byte, Byte> {
        if address < self.size { Ok(self.memory[address]) } else { Err(UCode::INVALID_MEMORY_READ) }
    }

    pub fn write_byte(&mut self, address: usize, byte: Byte) -> Result<(), Byte> {
        if address < self.size {
            self.memory[address] = byte;
            Ok(())
        } else { Err(UCode::INVALID_MEMORY_WRITE) }
    }
}