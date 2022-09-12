use std::borrow::Borrow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::{read, read_to_string};
use std::ops::{Deref, DerefMut};
use std::process::exit;
use std::sync::{Arc, Mutex};

use queues::{IsQueue, Queue};

use crate::lib::bus::bus::Bus;
use crate::lib::bus::bus_device::BusDevice;
use crate::lib::chip_util::{combine_to_double_word, combine_to_word};
use crate::lib::gpu::color::Color;
use crate::lib::gpu::monitor::Monitor;
use crate::lib::gpu::vector::Vector;
use crate::lib::mem::{Byte, DoubleWord, Word};
use crate::lib::ucode::gpu_assembly::GPUAssembly;
use crate::lib::ucode::ucode::UCode;

// XXXX'XXXX_XXXX'XXXX  -   WORD
// YYYY'YYYY_YYYY'YYYY  -   WORD
// CCCC'CCCC_CCCC'CCCC  -   WORD
// AXAX'AXAX_AXAX'AXAX  -   WORD
// AYAY'AYAY_AYAY'AYAY  -   WORD
// BBBB'BBBB            -   BYTE

//                          11 Bytes per vertex

// xx = x position u16
// yy = y position u16
// cc = color u16 [rrrr'gggg'bbbb'aaaa] 16bit color
// ax = x texture coordinate
// ay = y texture coordinate
// bb = z-index 256 layers

// BVB $0x0000_0000 $0x0000_0000            (vertex id, monitor id)
// VRX $0xXXXX'XXXX_XXXX'XXXX $0xYYYY'YYYY_YYYY'YYYY $RRRR_GGGG_BBBB_AAAA $0xAXAX'AXAX_AXAX'AXAX $0xAYAY'AYAY_AYAY'AYAY $BBBB_BBBB
// VRX $0xXXXX'XXXX_XXXX'XXXX $0xYYYY'YYYY_YYYY'YYYY $RRRR_GGGG_BBBB_AAAA $0xAXAX'AXAX_AXAX'AXAX $0xAYAY'AYAY_AYAY'AYAY $BBBB_BBBB
// VRX $0xXXXX'XXXX_XXXX'XXXX $0xYYYY'YYYY_YYYY'YYYY $RRRR_GGGG_BBBB_AAAA $0xAXAX'AXAX_AXAX'AXAX $0xAYAY'AYAY_AYAY'AYAY $BBBB_BBBB
// VRX $0xXXXX'XXXX_XXXX'XXXX $0xYYYY'YYYY_YYYY'YYYY $RRRR_GGGG_BBBB_AAAA $0xAXAX'AXAX_AXAX'AXAX $0xAYAY'AYAY_AYAY'AYAY $BBBB_BBBB
// UVB

pub struct GPU {
    address: Byte,
    instruction_buffer: Queue<Byte>,

    uuid: String,
    name: String,

    vertex_buffer: HashMap<Byte, Vec<Vector>>,
    display_buffer: Vec<Vec<Vec<Word>>>,

    vertex_buffer_pointer: Option<Byte>,
    monitor_write_pointer: Option<Byte>,
}

impl GPU {
    pub fn new(name: &str, uuid: &str) -> Self {
        GPU {
            address: 0x0,
            instruction_buffer: Queue::new(),
            uuid: uuid.to_string(),
            name: name.to_string(),

            vertex_buffer: HashMap::new(),
            display_buffer: vec![],

            vertex_buffer_pointer: None,

            monitor_write_pointer: None,
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
            self.queue_to_buffer(bus.lock().unwrap().poll(self.address));

            let x = self.fetch_instruction_byte();
            if x.is_ok() { instruction = x.unwrap(); } else { self.raise_exception(x.err().unwrap()) }
            let res = self.execute(instruction, displays);
            if res.is_ok() { finished_instruction = res.unwrap() } else { self.raise_exception(res.err().unwrap()) }
        }
    }

    fn raise_exception(&self, ucode: Byte) {
        println!("exception code: {:X} raised;\n{}", ucode, self.stack_trace());
        exit(ucode as i32)
    }

    fn stack_trace(&self) -> String {
        "[TODO] - todo!".to_string()
    }

    fn fetch_instruction_byte(&mut self) -> Result<Byte, Byte> {
        if self.instruction_buffer.size() <= 0 { return Ok(GPUAssembly::HLT); }
        let x = self.instruction_buffer.peek();
        if x.is_err() { return Err(UCode::INVALID_BUFFER_ACCESS); }
        let a = x.unwrap().to_owned();
        let r = self.instruction_buffer.remove();
        if r.is_err() { return Err(UCode::INVALID_BUFFER_ACCESS); }
        Ok(a)
    }
    fn fetch_instruction_word(&mut self) -> Result<Word, Byte> {
        let x1 = self.fetch_instruction_byte();
        if x1.is_err() { return Err(x1.err().unwrap()); }
        let x2 = self.fetch_instruction_byte();
        if x2.is_err() { return Err(x2.err().unwrap()); }
        Ok(combine_to_word(x1.unwrap(), x2.unwrap()))
    }
    fn fetch_instruction_double_word(&mut self) -> Result<DoubleWord, Byte> {
        let x1 = self.fetch_instruction_word();
        if x1.is_err() { return Err(x1.err().unwrap()); }
        let x2 = self.fetch_instruction_word();
        if x2.is_err() { return Err(x2.err().unwrap()); }
        Ok(combine_to_double_word(x1.unwrap(), x2.unwrap()))
    }

    fn write_word(&mut self, display: Byte, pixel_x: usize, pixel_y: usize, word: Word) -> Result<(), Byte> {
        let display = self.display_buffer.get_mut(display as usize);
        if display.is_none() { return Err(UCode::MONITOR_NOT_FOUND); }
        let row = display.unwrap().get_mut(pixel_x);
        if row.is_none() { return Err(UCode::PIXEL_OUT_OF_BOUNDS); }
        row.unwrap().insert(pixel_y, word);
        Ok(())
    }

    fn queue_to_buffer(&mut self, data: Vec<Byte>) {
        for i in data {
            let x = self.instruction_buffer.add(i);
            if x.is_err() { self.raise_exception(UCode::UNKNOWN_EXCEPTION) }
        }
    }

    fn coincide(&self, data: &Vec<Vector>, x: u16, y: u16) -> Option<Color> {
        None
    }
}

impl GPU {
    fn execute(&mut self, opcode: Byte, displays: &[&mut Monitor]) -> Result<bool, Byte> {
        match opcode {
            GPUAssembly::HLT => { Ok(true) }
            GPUAssembly::STK => {
                println!("{}", self.stack_trace());
                Ok(true)
            }
            GPUAssembly::BVB => {
                let x = self.fetch_instruction_byte();
                if x.is_err() { return Err(x.err().unwrap()); }
                self.vertex_buffer_pointer = Some(x.unwrap());

                let x = self.fetch_instruction_byte();
                if x.is_err() { return Err(x.err().unwrap()); }
                self.monitor_write_pointer = Some(x.unwrap());
                Ok(true)
            }
            GPUAssembly::UVB => {
                self.vertex_buffer_pointer = None;
                self.monitor_write_pointer = None;
                Ok(true)
            }
            GPUAssembly::VRX => {
                let x = self.fetch_instruction_word();
                if x.is_err() { return Err(x.err().unwrap()); }
                let y = self.fetch_instruction_word();
                if y.is_err() { return Err(y.err().unwrap()); }
                let c = self.fetch_instruction_word();
                if c.is_err() { return Err(c.err().unwrap()); }
                let tx = self.fetch_instruction_word();
                if tx.is_err() { return Err(tx.err().unwrap()); }
                let ty = self.fetch_instruction_word();
                if ty.is_err() { return Err(ty.err().unwrap()); }
                let z = self.fetch_instruction_byte();
                if z.is_err() { return Err(z.err().unwrap()); }

                let vertex = Vector::new(
                    self.monitor_write_pointer,
                    x.unwrap(),
                    y.unwrap(),
                    Some(c.unwrap()),
                    Some(tx.unwrap()),
                    Some(ty.unwrap()),
                    Some(z.unwrap()),
                );

                let x = self.vertex_buffer.get_mut(&self.vertex_buffer_pointer.unwrap());
                if x.is_none() { return Err(UCode::INVALID_BUFFER_ACCESS); }
                x.unwrap().push(vertex);
                Ok(true)
            }
            GPUAssembly::DRW => {
                let mut to_write: HashMap<Byte, Vec<(usize, usize, Word)>> = HashMap::new();
                for i in self.vertex_buffer.iter() {
                    let monitor = i.1.first().unwrap().monitor;
                    let mut m = &displays[monitor as usize];
                    for x in 0..m.width().to_owned() {
                        for y in 0..m.height().to_owned() {
                            let coincide = self.coincide(i.1, x, y);
                            if coincide.is_none() { continue; }

                            to_write.insert(monitor, vec![]);
                            to_write.get_mut(&monitor).unwrap().push((x as usize, y as usize, coincide.unwrap().as_word()));
                            // cant borrow self as mutable as I am iterating it (having a reference)
                            // let w = self.write_word(monitor, x as usize, y as usize, coincide.unwrap().as_word());
                            // if w.is_err() { return Err(w.err().unwrap()); }
                            m.deref_mut().write(x, y, coincide.unwrap().as_word());
                        }
                    }
                }
                Ok(true)
            }
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