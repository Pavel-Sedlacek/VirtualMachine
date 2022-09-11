use std::sync::Mutex;

use crate::lib::bus::Bus;
use crate::lib::gpu::monitor::Monitor;
use crate::lib::mem::Byte;
use crate::lib::ucode::gpu_assembly::GPUAssembly;

pub struct GPU {
    buffer: [Byte; 256],
    buffer_pointer: usize,
}

impl GPU {
    pub fn new() -> Self {
        GPU {
            buffer: [GPUAssembly::HLT; 256],
            buffer_pointer: 0x0,
        }
    }
}

impl GPU {
    pub fn launch(&mut self, bus: &mut Mutex<Bus>, displays: &[&mut Monitor]) {}
}