use std::thread;

use crate::lib::chip_util::{combine_to_double_word, combine_to_word};
use crate::lib::mem::{B, Byte, DoubleWord, Word};
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
    instruction_step_a_registry_long: DoubleWord,

    stack_pointer: DoubleWord,
    program_counter: DoubleWord,
}

/// memory for primitives (ints, chars, floats, ...)
/// stack   =>  0x0000'0000     -   0x04FF'FFFF     -       (640 MB)

/// memory for objects
/// heap    =>   0x0500'0000    -   0x0FFF'FFF     -        (1408 MB)

/// memory for storing program bytecode
/// prg     =>  0x1000'0000     -   0x1FFF'FFFF     -       (2048 MB)

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
            instruction_step_a_registry_long: 0x0
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
    fn fetch_double_word(&mut self, ram: &mut RAM) -> Result<DoubleWord, Byte> {
        let sig = self.fetch_word(ram);
        if sig.is_ok() { self.on_success_byte_fetch() } else { return Err(sig.err().unwrap()); };
        let insig = self.fetch_word(ram);
        if insig.is_ok() { self.on_success_byte_fetch() } else { return Err(insig.err().unwrap()); };
        Ok(combine_to_double_word(sig.unwrap(), insig.unwrap()))
    }

    fn on_success_byte_fetch(&mut self) {
        self.program_counter += 1;
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

    fn raise_exception(&self, ucode: Byte) {
        panic!("exception code: {} raised;\n{}", ucode, self.stack_trace())
    }

    fn stack_trace(&self) -> String {
        format!("a registry:   {}\nx registry:   {}", self.a_register, self.x_register)
    }
}

impl CPU {
    fn execute(&mut self, opcode: u8, ram: &mut RAM) -> Result<bool, Byte> {
        match opcode {
            Assembly::HLT => { Ok(true) }

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
            Assembly::LDX => {
                match self.instruction_step {
                    0 => {
                        let x = self.fetch_word(ram);
                        if x.is_ok() { self.instruction_step_a_registry = x.unwrap() } else { return Err(x.err().unwrap()); }
                    }
                    1 => self.x_register = self.instruction_step_a_registry,
                    _ => ()
                }
                Ok(self.instruction_step >= 1)
            }
            Assembly::LDY => {
                match self.instruction_step {
                    0 => {
                        let x = self.fetch_word(ram);
                        if x.is_ok() { self.instruction_step_a_registry = x.unwrap() } else { return Err(x.err().unwrap()); }
                    }
                    1 => self.y_register = self.instruction_step_a_registry,
                    _ => ()
                }
                Ok(self.instruction_step >= 1)
            }

            Assembly::TAX => {
                self.x_register = self.a_register;
                Ok(true)
            }
            Assembly::TAY => {
                self.y_register = self.a_register;
                Ok(true)
            }
            Assembly::TXA => {
                self.a_register = self.x_register;
                Ok(true)
            }
            Assembly::TXY => {
                self.y_register = self.x_register;
                Ok(true)
            }
            Assembly::TYA => {
                self.a_register = self.y_register;
                Ok(true)
            }
            Assembly::TYX => {
                self.x_register = self.y_register;
                Ok(true)
            }

            Assembly::STA => {
                match self.instruction_step {
                    0 => {
                        let x = self.fetch_double_word(ram);
                        if x.is_ok() { self.instruction_step_a_registry_long = x.unwrap() } else { return Err(x.err().unwrap()); }
                    }
                    1 => {
                        let x = ram.write_word(self.instruction_step_a_registry_long as usize, self.a_register);
                        if x.is_err() { return Err(x.err().unwrap()); }
                    }
                    _ => ()
                }
                Ok(self.instruction_step >= 1)
            }
            Assembly::STX => {
                match self.instruction_step {
                    0 => {
                        let x = self.fetch_double_word(ram);
                        if x.is_ok() { self.instruction_step_a_registry_long = x.unwrap() } else { return Err(x.err().unwrap()); }
                    }
                    1 => {
                        let x = ram.write_word(self.instruction_step_a_registry_long as usize, self.x_register);
                        if x.is_err() { return Err(x.err().unwrap()); }
                    }
                    _ => ()
                }
                Ok(self.instruction_step >= 1)
            }
            Assembly::STY => {
                match self.instruction_step {
                    0 => {
                        let x = self.fetch_double_word(ram);
                        if x.is_ok() { self.instruction_step_a_registry_long = x.unwrap() } else { return Err(x.err().unwrap()); }
                    }
                    1 => {
                        let x = ram.write_word(self.instruction_step_a_registry_long as usize, self.y_register);
                        if x.is_err() { return Err(x.err().unwrap()); }
                    }
                    _ => ()
                }
                Ok(self.instruction_step >= 1)
            }

            Assembly::PSA => Ok(false),
            Assembly::PSX => Ok(false),
            Assembly::PSY => Ok(false),
            Assembly::PSP => Ok(false),

            Assembly::PLA => Ok(false),
            Assembly::PLX => Ok(false),
            Assembly::PLY => Ok(false),
            Assembly::PLP => Ok(false),

            Assembly::CMA => {
                match self.instruction_step {
                    0 => {
                        let x = self.fetch_word(ram);
                        if x.is_ok() { self.instruction_step_a_registry = x.unwrap() } else { return Err(x.err().unwrap()); }
                    }
                    1 => self.flag_register = if self.a_register == self.instruction_step_a_registry { self.flag_register.set_bit(1) } else { self.flag_register.unset_bit(1) },
                    _ => ()
                }
                Ok(self.instruction_step >= 1)
            }
            Assembly::CMX => {
                match self.instruction_step {
                    0 => {
                        let x = self.fetch_word(ram);
                        if x.is_ok() { self.instruction_step_a_registry = x.unwrap() } else { return Err(x.err().unwrap()); }
                    }
                    1 => self.flag_register = if self.x_register == self.instruction_step_a_registry { self.flag_register.set_bit(1) } else { self.flag_register.unset_bit(1) },
                    _ => ()
                }
                Ok(self.instruction_step >= 1)
            }
            Assembly::CMY => {
                match self.instruction_step {
                    0 => {
                        let x = self.fetch_word(ram);
                        if x.is_ok() { self.instruction_step_a_registry = x.unwrap() } else { return Err(x.err().unwrap()); }
                    }
                    1 => self.flag_register = if self.y_register == self.instruction_step_a_registry { self.flag_register.set_bit(1) } else { self.flag_register.unset_bit(1) },
                    _ => ()
                }
                Ok(self.instruction_step >= 1)
            }
            Assembly::CAX => {
                self.flag_register = if self.x_register == self.a_register { self.flag_register.set_bit(1) } else { self.flag_register.unset_bit(1) };
                Ok(true)
            }
            Assembly::CAY => {
                self.flag_register = if self.a_register == self.y_register { self.flag_register.set_bit(1) } else { self.flag_register.unset_bit(1) };
                Ok(true)
            }
            Assembly::CXY => {
                self.flag_register = if self.x_register == self.y_register { self.flag_register.set_bit(1) } else { self.flag_register.unset_bit(1) };
                Ok(true)
            }

            _ => Ok(true)
        }
    }
}