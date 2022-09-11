use std::borrow::Borrow;
use std::collections::HashMap;
use std::iter::Map;
use std::ops::Deref;
use std::path::Iter;

use crate::{CPUAssembly, RAM};
use crate::lib::mem::Byte;
use crate::lib::ucode::gpu_assembly::GPUAssembly;

pub struct Bus {
    buffer: HashMap<Byte, Vec<Byte>>,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            buffer: HashMap::new()
        }
    }
}

impl Bus {
    pub fn write(&mut self, address: Byte, byte: Byte) {
        let mut x = self.buffer.get_mut(address.borrow());
        if x.is_none() { return; }
        x.unwrap().push(byte);
    }

    pub fn poll(&mut self, address: Byte) -> Vec<Byte> {
        let mut x = self.buffer.get_mut(address.borrow());
        if x.is_none() { return vec![]; }
        // TODO clear bus buffer
        x.unwrap().to_vec()
    }

    pub fn register(&mut self) -> Byte {
        3
    }
}