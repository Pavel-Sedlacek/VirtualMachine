use std::borrow::Borrow;
use std::collections::HashMap;
use std::iter::Map;
use std::ops::Deref;
use std::path::Iter;

use crate::{CPUAssembly, RAM};
use crate::lib::bus::bus_device::BusDevice;
use crate::lib::mem::Byte;
use crate::lib::ucode::gpu_assembly::GPUAssembly;

pub struct Bus<'a> {
    buffer: HashMap<Byte, Vec<Byte>>,
    devices: HashMap<Byte, Box<&'a dyn BusDevice>>,
    pointer: Byte,
}

impl<'a> Bus<'a> {
    pub fn new() -> Self {
        Bus {
            buffer: HashMap::new(),
            devices: HashMap::new(),
            pointer: 0x0,
        }
    }
}

impl<'a> Bus<'a> {
    pub fn write(&mut self, address: Byte, byte: Byte) {
        let mut x = self.buffer.get_mut(address.borrow());
        if x.is_none() { return; }
        x.unwrap().push(byte);
    }

    pub fn poll(&mut self, address: Byte) -> Vec<Byte> {
        let mut x = self.buffer.get_mut(address.borrow());
        if x.is_none() { return vec![]; }
        let a = x.unwrap().clone();
        self.buffer.insert(address, vec![]);
        return a;
    }

    pub fn register(&mut self, device: Box<&'a dyn BusDevice>) -> Byte {
        self.buffer.insert(self.pointer, vec![]);
        self.devices.insert(self.pointer, device);
        self.pointer += 1;
        self.pointer
    }

    pub fn devices(&self) -> String {
        let mut x = "";
        for i in self.devices {
            x.to_owned().push_str(format!("{}: {} [{}]", i.0, i.1.uuid(), i.1.name()).as_str());
        }
        x.to_string()
    }
}