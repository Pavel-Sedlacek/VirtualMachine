use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

use crate::lib::bus::bus::Bus;
use crate::lib::bus::bus_device::BusDevice;
use crate::lib::gpu::monitor::Monitor;
use crate::lib::mem::Byte;
use crate::lib::ucode::gpu_assembly::GPUAssembly;

pub struct GPU {
    buffer: [Byte; 256],
    buffer_pointer: usize,
    uuid: String,
    name: String,
}

impl GPU {
    pub fn new(name: &str, uuid: &str) -> Self {
        GPU {
            buffer: [GPUAssembly::HLT; 256],
            buffer_pointer: 0x0,
            uuid: uuid.to_string(),
            name: name.to_string(),
        }
    }
}

impl GPU {
    pub fn launch(&mut self, bus: &Arc<Mutex<Bus>>, displays: &[&mut Monitor]) {
        bus.lock().unwrap().register(Box::new(self));
        loop {
            let x = bus.lock().unwrap().poll(0x0);
            if !x.is_empty() {
                for i in x.iter() {
                    println!("byte: {}", i)
                }
            }
        }
    }
}

impl BusDevice for GPU {
    fn uuid(&self) -> String {
        self.uuid.to_string()
    }

    fn name(&self) -> String {
        self.name.to_string()
    }
}