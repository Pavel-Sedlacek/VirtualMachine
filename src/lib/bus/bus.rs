use std::borrow::Borrow;
use std::collections::{BTreeMap, HashMap};

use crate::lib::bus::bus_device::BusDevice;
use crate::lib::mem::Byte;

struct BusDeviceInfo {
    uuid: String,
    name: String
}

pub struct Bus {
    buffer: BTreeMap<Byte, Vec<Byte>>,
    devices: BTreeMap<Byte, BusDeviceInfo>,
    pointer: Byte,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            buffer: BTreeMap::new(),
            devices: BTreeMap::new(),
            pointer: 0x0,
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
        let a = x.unwrap().clone();
        self.buffer.insert(address, vec![]);
        return a;
    }

    pub fn register(&mut self, device: Box<&dyn BusDevice>) -> Byte {
        self.buffer.insert(self.pointer, vec![]);
        self.devices.insert(self.pointer, BusDeviceInfo {uuid: device.uuid(), name: device.uuid()});
        self.pointer += 1;
        self.pointer
    }

    pub fn devices(&self) -> String {
        let mut x = "".to_string();
        for i in self.devices.iter() {
            x += format!("{:#04X}: {} [{}]\n", i.0, i.1.uuid, i.1.name).as_str();
        }
        x.to_string()
    }
}