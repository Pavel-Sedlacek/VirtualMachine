use crate::lib::mem::{Byte, Word};

pub struct Vector {
    pub monitor: Byte,

    pub x: Word,
    pub y: Word,

    pub c: Word,

    pub tx: Word,
    pub ty: Word,

    pub z: Byte,
}

impl Vector {
    pub fn new(monitor: Option<Byte>, x: Word, y: Word, c: Option<Word>, tx: Option<Word>, ty: Option<Word>, z: Option<Byte>) -> Self {
        Vector {
            monitor: monitor.unwrap_or(0x0),
            x,
            y,
            c: c.unwrap_or(0b1111_1111_1111_1111),
            tx: tx.unwrap_or(0b0000_0000_0000_0000),
            ty: ty.unwrap_or(0b0000_0000_0000_0000),
            z: z.unwrap_or(0x00),
        }
    }
}