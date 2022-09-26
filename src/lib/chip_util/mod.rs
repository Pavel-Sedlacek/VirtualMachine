use std::ops::Range;
use std::sync::{LockResult, Mutex, MutexGuard};

use crate::lib::mem::{Byte, DoubleWord, Word};

pub fn combine_to_word(significant_byte: Byte, insignificant_byte: Byte) -> Word {
    ((significant_byte as u16) << 8) | (insignificant_byte) as Word
}

pub fn combine_to_double_word(significant_word: Word, insignificant_word: Word) -> DoubleWord {
    ((significant_word as u32) << 16) | (insignificant_word) as DoubleWord
}

pub fn map(value: u16, from: Range<u16>, to: Range<u8>) -> u8 {
    (to.start as f32 + (value as f32 / (from.end as f32 - from.start as f32)) * (to.end as f32 - to.start as f32)) as u8
}

pub trait BlockingLock<T> {
    fn b_lock(&self) -> MutexGuard<'_, T>;
}

impl<X> BlockingLock<X> for Mutex<X> {
    fn b_lock(&self) -> MutexGuard<'_, X> {
        loop {
            let x = self.lock();
            if x.is_ok() { return x.unwrap(); }
        }
    }
}