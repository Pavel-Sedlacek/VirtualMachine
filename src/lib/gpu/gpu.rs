use std::borrow::Borrow;
use std::fs::read;
use std::ops::{Deref, DerefMut};
use std::process::exit;
use std::sync::{Arc, Mutex};

use crate::lib::bus::bus::Bus;
use crate::lib::bus::bus_device::BusDevice;
use crate::lib::chip_util::{combine_to_double_word, combine_to_word};
use crate::lib::gpu::monitor::Monitor;
use crate::lib::mem::{Byte, DoubleWord, Word};
use crate::lib::ucode::gpu_assembly::GPUAssembly;
use crate::lib::ucode::ucode::UCode;

pub struct GPU {
    address: Byte,
    instruction_buffer: [Byte; 256],
    instruction_buffer_pointer: usize,

    uuid: String,
    name: String,

    display_buffer: Vec<Vec<Vec<Byte>>>,

    instruction_step: Byte,
    instruction_step_a_registry: Word,
    instruction_step_a_registry_long: DoubleWord,

    a_register: Word,
    x_register: Word,
    y_register: Word,
}

impl GPU {
    pub fn new(name: &str, uuid: &str) -> Self {
        GPU {
            address: 0x0,
            instruction_buffer: [GPUAssembly::HLT; 256],
            instruction_buffer_pointer: 0x0,
            uuid: uuid.to_string(),
            name: name.to_string(),
            display_buffer: vec![],

            a_register: 0x0,
            x_register: 0x0,
            y_register: 0x0,

            instruction_step: 0x0,
            instruction_step_a_registry: 0x0,
            instruction_step_a_registry_long: 0x0,
        }
    }
}

impl GPU {

    pub fn launch(&mut self, bus: &Arc<Mutex<Bus>>, displays: &[&mut Monitor]) {
        self.address = bus.lock().unwrap().register(Box::new(self));

        self.display_buffer = vec![vec![]; displays.len()];
        for d in displays.iter() {
            self.display_buffer.push(vec![vec![0x0; d.width() as usize]; d.height() as usize])
        }

        let mut instruction: Byte = GPUAssembly::HLT;
        let mut finished_instruction: bool = true;

        loop {
            self.append_to_buffer(bus.lock().unwrap().poll(self.address));

            if finished_instruction {
                let x = self.fetch_instruction_byte();
                if x.is_ok() {
                    instruction = x.unwrap();
                    self.on_successful_instruction_fetch();
                } else { self.raise_exception(x.err().unwrap()) }
            }
            let res = self.execute(instruction);
            self.instruction_step += 1;
            if res.is_ok() { finished_instruction = res.unwrap() } else { self.raise_exception(res.err().unwrap()) }
            if finished_instruction { self.instruction_step = 0 }
        }
    }

    fn raise_exception(&self, ucode: Byte) {
        println!("exception code: {:X} raised;\n{}", ucode, self.stack_trace());
        exit(ucode as i32)
    }

    fn stack_trace(&self) -> String {
        "[TODO] - todo!".to_string()
    }

    fn on_successful_instruction_fetch(&mut self) {
        self.instruction_buffer_pointer -= 1;
    }

    fn fetch_instruction_byte(&mut self) -> Result<Byte, Byte> {
        if self.instruction_buffer.len() <= 0 { return Err(UCode::INVALID_BUFFER_ACCESS); }
        let x = self.instruction_buffer.get(self.instruction_buffer_pointer);
        if x.is_none() { return Err(UCode::INVALID_BUFFER_ACCESS); }
        self.instruction_buffer_pointer -= 1;
        Ok(*x.unwrap())
    }
    fn fetch_instruction_word(&mut self) -> Result<Word, Byte> {
        if self.instruction_buffer.len() <= 0 { return Err(UCode::INVALID_BUFFER_ACCESS); }
        let x1 = self.fetch_instruction_byte();
        if x1.is_err() { return Err(x1.err().unwrap()); }
        let x2 = self.fetch_instruction_byte();
        if x2.is_err() { return Err(x2.err().unwrap()); }
        Ok(combine_to_word(x1.unwrap(), x2.unwrap()))
    }
    fn fetch_instruction_double_word(&mut self) -> Result<DoubleWord, Byte> {
        if self.instruction_buffer.len() <= 0 { return Err(UCode::INVALID_BUFFER_ACCESS); }
        let x1 = self.fetch_instruction_word();
        if x1.is_err() { return Err(x1.err().unwrap()); }
        let x2 = self.fetch_instruction_word();
        if x2.is_err() { return Err(x2.err().unwrap()); }
        Ok(combine_to_double_word(x1.unwrap(), x2.unwrap()))
    }

    fn write_byte(&mut self, display: usize, pixel_x: usize, pixel_y: usize, byte: Byte) -> Result<(), Byte> {
        let display = self.display_buffer.get(display);
        if display.is_none() { return Err(UCode::MONITOR_NOT_FOUND); }
        let row = display.unwrap().get(pixel_x);
        if row.is_none() { return Err(UCode::PIXEL_OUT_OF_BOUNDS); }
        let pixel = row.unwrap().insert(pixel_y, byte);
        Ok(())
    }

    fn append_to_buffer(&mut self, data: Vec<Byte>) {
        for i in data {
            self.instruction_buffer_pointer += 1;
            self.instruction_buffer[self.instruction_buffer_pointer] = i;
        }
    }
}

impl GPU {
    fn execute(&mut self, opcode: Byte) -> Result<bool, Byte> {
        match opcode {
            GPUAssembly::HLT => { Ok(true) }
            _ => { Ok(true) }
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