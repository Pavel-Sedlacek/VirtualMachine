use std::thread;

use crate::lib::chip_util::combine_to_word;
use crate::lib::mem::{Byte, DoubleWord, Word};
use crate::lib::ucode::assembly::Assembly;
use crate::RAM;

pub struct CPU {
    a_register: Word,
    x_register: Word,
    y_register: Word,

    /// negative, overflow, -, break, decimal, interrupt, zero, carry,
    flag_register: Byte,

    /// instruction step to execute, max 8 steps per instructions
    /// each instruction takes the same amount of time as each instruction has to complete all 8 clock cycles
    instruction_step: u8,

    instruction_step_a_registry: Word,

    stack_pointer: DoubleWord,
    program_counter: DoubleWord,
}


// FIXME SHIT this does not add up
/// stack pointer        0x0000'0000    -    0x0000'0ffe     (4096       Bytes) ~ 4   kB
/// memory alloc         0x0000'0fff    -    0x0fff'feee     (16838400   Bytes) ~ 128 MB
/// program alloc        0x0fff'ff00    -    0xffff'feee     (4311744000 Bytes) ~ 30  GB
/// program counter      0xffff'ff00    -    0xffff'ffff     (256        Bytes) ~ 1/4 kB
impl CPU {
    pub fn new() -> Self {
        CPU {
            a_register: 0x0,
            x_register: 0x0,
            y_register: 0x0,
            flag_register: 0x0,
            stack_pointer: 0x0,
            program_counter: 0xffff_fff0,
            instruction_step: 0,
            instruction_step_a_registry: 0x0,
        }
    }

    fn fetch_byte(&mut self, ram: &mut RAM) -> Result<Byte, Byte> {
        while ram.is_locked() {};
        ram.lock().unwrap();
        let res = ram.fetch_byte(self.program_counter as usize);
        ram.unlock().unwrap();
        res
    }
    fn fetch_word(&mut self, ram: &mut RAM) -> Result<Word, Byte> {
        let sig = self.fetch_byte(ram);
        if sig.is_ok() { self.on_success_byte_fetch() } else { return Err(sig.err().unwrap()); };
        let insig = self.fetch_byte(ram);
        if insig.is_ok() { self.on_success_byte_fetch() } else { return Err(insig.err().unwrap()); };
        Ok(combine_to_word(sig.unwrap(), insig.unwrap()))
    }
    fn on_success_byte_fetch(&mut self) {
        self.program_counter += 1;
    }

    fn execute(&mut self, opcode: u8, ram: &mut RAM) -> Result<bool, Byte> {
        match opcode {
            Assembly::LDA => {
                match self.instruction_step {
                    0 => {
                        let x = self.fetch_word(ram);
                        if x.is_ok() { self.instruction_step_a_registry = x.unwrap() } else { return Err(x.err().unwrap()); }
                    }
                    1 => self.a_register = self.instruction_step_a_registry,
                    _ => ()
                }
                Ok(self.instruction_step >= 1)
            }
            Assembly::OUA => Ok(false),

            _ => Ok(true),
        }
    }

    pub fn launch(&mut self, ram: &mut RAM) {
        let mut instruction: Byte = Assembly::HLT;
        let mut finished_instruction: bool = true;

        loop {
            if finished_instruction {
                let x = self.fetch_byte(ram);
                if x.is_ok() {
                    instruction = x.unwrap();
                    self.on_success_byte_fetch()
                } else { self.raise_exception(x.err().unwrap()) }
            }
            let res = self.execute(instruction, ram);
            self.instruction_step += 1;
            if res.is_ok() { finished_instruction = res.unwrap() } else { self.raise_exception(res.err().unwrap()) }
            println!("{}", self.stack_trace())
        }
    }

    // TODO
    pub fn raise_exception(&self, ucode: Byte) {
        panic!("exception code: {} raised;\n{}", ucode, self.stack_trace())
    }

    fn stack_trace(&self) -> String {
        format!("a registry:   {}\nx registry:   {}", self.a_register, self.x_register)
    }
}