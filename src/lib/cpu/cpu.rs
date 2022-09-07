use std::num::FpCategory::Zero;
use std::process::exit;
use std::ptr::addr_of;
use std::thread;

use crate::lib::chip_util::{combine_to_double_word, combine_to_word};
use crate::lib::mem::{B, Byte, D, DoubleWord, W, Word};
use crate::lib::ucode::assembly::Assembly;
use crate::RAM;

pub struct CPU {
    a_register: Word,
    x_register: Word,
    y_register: Word,

    flag_register: Byte,

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
/// program =>   0x1000'0000     -   0x1FFF'FFFF     -       (2048 MB)

impl CPU {
    pub fn new() -> Self {
        CPU {
            a_register: 0x0,
            x_register: 0x0,
            y_register: 0x0,
            flag_register: 0x0,
            stack_pointer: 0x04FF_FFFF,
            program_counter: 0x1000_0000,
            instruction_step: 0,
            instruction_step_a_registry: 0x0,
            instruction_step_a_registry_long: 0x0,
        }
    }

    const CARRY: usize = 0;
    const ZERO: usize = 1;
    const INTERRUPT: usize = 2;
    const DECIMAL: usize = 3;
    const BREAK: usize = 4;
    const OVERFLOW: usize = 6;
    const NEGATIVE: usize = 7;

    fn fetch_byte(&mut self, ram: &mut RAM) -> Result<Byte, Byte> {
        while ram.is_locked() {};
        ram.lock().unwrap();
        let res = ram.fetch_byte(self.stack_pointer as usize);
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
        if sig.is_err() { return Err(sig.err().unwrap()) }
        let insig = self.fetch_word(ram);
        if insig.is_err() { return Err(insig.err().unwrap()) }
        Ok(combine_to_double_word(sig.unwrap(), insig.unwrap()))
    }


    fn read_byte(&mut self, ram: &mut RAM, address: usize) -> Result<Byte, Byte> {
        while ram.is_locked() {};
        ram.lock().unwrap();
        let res = ram.fetch_byte(address);
        ram.unlock().unwrap();
        res
    }
    fn read_word(&mut self, ram: &mut RAM, address: usize) -> Result<Word, Byte> {
        let sig = self.read_byte(ram, address);
        if sig.is_err() { return Err(sig.err().unwrap()); };
        let insig = self.read_byte(ram, address + 1);
        if insig.is_err() { return Err(insig.err().unwrap()); };
        Ok(combine_to_word(sig.unwrap(), insig.unwrap()))
    }
    fn read_double_word(&mut self, ram: &mut RAM, address: usize) -> Result<DoubleWord, Byte> {
        let sig = self.read_word(ram, address);
        if sig.is_err() { return Err(sig.err().unwrap()) }
        let insig = self.read_word(ram, address + 2);
        if insig.is_err() { return Err(insig.err().unwrap()) }
        Ok(combine_to_double_word(sig.unwrap(), insig.unwrap()))
    }

    fn write_byte(&mut self, ram: &mut RAM, address: usize, byte: Byte) -> Result<(), Byte> {
        while ram.is_locked() {};
        ram.lock().unwrap();
        let res = ram.write_byte(address, byte);
        ram.unlock().unwrap();
        if res.is_err() { return Err(res.err().unwrap()) }
        Ok(())
    }
    fn write_word(&mut self, ram: &mut RAM, address: usize, word: Word) -> Result<(), Byte> {
        while ram.is_locked() {};
        ram.lock().unwrap();
        let res = self.write_byte(ram, address, word.significant_byte());
        let res2 = self.write_byte(ram, address + 1, word.significant_byte());
        ram.unlock().unwrap();
        if res.is_err() { return Err(res.err().unwrap()) }
        if res2.is_err() { return Err(res2.err().unwrap()) }
        Ok(())
    }
    fn write_double_word(&mut self, ram: &mut RAM, address: usize, dword: DoubleWord) -> Result<(), Byte> {
        while ram.is_locked() {};
        ram.lock().unwrap();
        let res = self.write_word(ram, address, dword.significant_word());
        let res2 = self.write_word(ram, address + 1, dword.insignificant_word());
        ram.unlock().unwrap();
        if res.is_err() { return Err(res.err().unwrap()) }
        if res2.is_err() { return Err(res2.err().unwrap()) }
        Ok(())
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
            if finished_instruction { self.instruction_step = 0 }
            println!("{}", self.stack_trace())
        }
    }

    fn raise_exception(&self, ucode: Byte) {
        println!("exception code: {} raised;\n{}", format!("{:X}", ucode), self.stack_trace());
        exit(ucode as i32)
    }

    fn stack_trace(&self) -> String {
        format!("-----------------------\n\
        a      :     {}\n\
        x      :     {}\n\
        y      :     {}\n\
        flag   :     {}\n\
        program:     {}\n\
        stack  :     {}",
                self.a_register,
                self.x_register,
                self.y_register,
                format!("{:0>8}", format!("{:X}", self.flag_register)),
                format!("{:0>8}", format!("{:X}", self.program_counter)),
                format!("{:0>8}", format!("{:X}", self.stack_pointer)))
    }
}

impl CPU {
    fn execute(&mut self, opcode: u8, ram: &mut RAM) -> Result<bool, Byte> {
        match opcode {
            Assembly::HLT => { Ok(true) }

            // TODO remove
            Assembly::STK => {
                println!("{}", self.stack_trace());
                Ok(true)
            }

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
                        let x = self.write_word(ram, self.instruction_step_a_registry_long as usize, self.a_register);
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
                        let x = self.write_word(ram, self.instruction_step_a_registry_long as usize, self.x_register);
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
                        let x = self.write_word(ram, self.instruction_step_a_registry_long as usize, self.y_register);
                        if x.is_err() { return Err(x.err().unwrap()); }
                    }
                    _ => ()
                }
                Ok(self.instruction_step >= 1)
            }

            // TODO add load from memory ? storing is pointless otherwise

            Assembly::PSA => {
                let res = self.write_word(ram, self.stack_pointer as usize, self.a_register);
                if res.is_err() { return Err(res.err().unwrap()) }
                self.stack_pointer -= 2;
                Ok(true)
            },
            Assembly::PSX => {
                let res = self.write_word(ram, self.stack_pointer as usize, self.x_register);
                if res.is_err() { return Err(res.err().unwrap()) }
                self.stack_pointer -= 2;
                Ok(true)
            },
            Assembly::PSY => {
                let res = self.write_word(ram, self.stack_pointer as usize, self.y_register);
                if res.is_err() { return Err(res.err().unwrap()) }
                self.stack_pointer -= 2;
                Ok(true)
            },
            Assembly::PSP => {
                let res = self.write_double_word(ram, self.stack_pointer as usize, self.program_counter);
                if res.is_err() { return Err(res.err().unwrap()) }
                self.stack_pointer -= 4;
                Ok(true)
            },

            Assembly::PLA => {
                let res = self.read_word(ram, self.stack_pointer as usize);
                if res.is_err() { return Err(res.err().unwrap()) } else { self.a_register = res.unwrap() }
                self.stack_pointer += 2;
                Ok(true)
            },
            Assembly::PLX => {
                let res = self.read_word(ram, self.stack_pointer as usize);
                if res.is_err() { return Err(res.err().unwrap()) } else { self.x_register = res.unwrap() }
                self.stack_pointer += 2;
                Ok(true)
            },
            Assembly::PLY => {
                let res = self.read_word(ram, self.stack_pointer as usize);
                if res.is_err() { return Err(res.err().unwrap()) } else { self.y_register = res.unwrap() }
                self.stack_pointer += 2;
                Ok(true)
            },
            Assembly::PLP => {
                let res = self.read_double_word(ram, self.stack_pointer as usize);
                if res.is_err() { return Err(res.err().unwrap()) } else { self.program_counter = res.unwrap() }
                self.stack_pointer += 4;
                Ok(true)
            },

            Assembly::CMP => {
                match self.instruction_step {
                    0 => {
                        let x = self.fetch_word(ram);
                        if x.is_ok() { self.instruction_step_a_registry = x.unwrap() } else { return Err(x.err().unwrap()); }
                    }
                    1 => self.flag_register =
                        if self.a_register == self.instruction_step_a_registry { self.flag_register.set_bit(CPU::ZERO) } else { self.flag_register.unset_bit(CPU::ZERO) },
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
                    1 => self.flag_register =
                        if self.x_register == self.instruction_step_a_registry { self.flag_register.set_bit(CPU::ZERO) } else { self.flag_register.unset_bit(CPU::ZERO) },
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
                    1 => self.flag_register =
                        if self.y_register == self.instruction_step_a_registry { self.flag_register.set_bit(CPU::ZERO) } else { self.flag_register.unset_bit(CPU::ZERO) },
                    _ => ()
                }
                Ok(self.instruction_step >= 1)
            }
            Assembly::CAX => {
                self.flag_register = if self.x_register == self.a_register { self.flag_register.set_bit(CPU::ZERO) } else { self.flag_register.unset_bit(CPU::ZERO) };
                Ok(true)
            }
            Assembly::CAY => {
                self.flag_register = if self.a_register == self.y_register { self.flag_register.set_bit(CPU::ZERO) } else { self.flag_register.unset_bit(CPU::ZERO) };
                Ok(true)
            }
            Assembly::CXY => {
                self.flag_register = if self.x_register == self.y_register { self.flag_register.set_bit(CPU::ZERO) } else { self.flag_register.unset_bit(CPU::ZERO) };
                Ok(true)
            }

            Assembly::BEQ => {
                if self.flag_register.is_set_bit(CPU::ZERO) {
                    match self.instruction_step {
                        0 => {
                            let x = self.fetch_double_word(ram);
                            if x.is_ok() { self.instruction_step_a_registry_long = x.unwrap() } else { return Err(x.err().unwrap()); }
                        }
                        1 => self.program_counter = self.instruction_step_a_registry_long,
                        _ => ()
                    }
                    Ok(self.instruction_step >= 1)
                } else { Ok(true) }
            }
            Assembly::BNE => {
                if !self.flag_register.is_set_bit(CPU::ZERO) {
                    match self.instruction_step {
                        0 => {
                            let x = self.fetch_double_word(ram);
                            if x.is_ok() { self.instruction_step_a_registry_long = x.unwrap() } else { return Err(x.err().unwrap()); }
                        }
                        1 => self.program_counter = self.instruction_step_a_registry_long,
                        _ => ()
                    }
                    Ok(self.instruction_step >= 1)
                } else { Ok(true) }
            }

            Assembly::JMP => {
                match self.instruction_step {
                    0 => {
                        let x = self.fetch_double_word(ram);
                        if x.is_ok() { self.instruction_step_a_registry_long = x.unwrap() } else { return Err(x.err().unwrap()); }
                    }
                    1 => self.program_counter = self.instruction_step_a_registry_long,
                    _ => ()
                }
                Ok(self.instruction_step >= 1)
            }

            Assembly::DEC => {
                if self.a_register == Word::MIN { self.flag_register.set_bit(CPU::OVERFLOW); }
                self.a_register -= 1;
                if self.a_register < 0 { self.flag_register.set_bit(CPU::NEGATIVE); }
                Ok(true)
            }
            Assembly::DEX => {
                if self.x_register == Word::MIN { self.flag_register.set_bit(CPU::OVERFLOW); }
                self.x_register -= 1;
                if self.x_register < 0 { self.flag_register.set_bit(CPU::NEGATIVE); }
                Ok(true)
            }
            Assembly::DEY => {
                if self.y_register == Word::MIN { self.flag_register.set_bit(CPU::OVERFLOW); }
                self.y_register -= 1;
                if self.y_register < 0 { self.flag_register.set_bit(CPU::NEGATIVE); }
                Ok(true)
            }

            Assembly::INC => {
                if self.a_register == Word::MAX { self.flag_register.set_bit(CPU::OVERFLOW); }
                self.a_register += 1;
                if self.a_register < 0 { self.flag_register.set_bit(CPU::NEGATIVE); }
                Ok(true)
            }
            Assembly::INX => {
                if self.x_register == Word::MAX { self.flag_register.set_bit(CPU::OVERFLOW); }
                self.x_register += 1;
                if self.x_register < 0 { self.flag_register.set_bit(CPU::NEGATIVE); }
                Ok(true)
            }
            Assembly::INY => {
                if self.y_register == Word::MAX { self.flag_register.set_bit(CPU::OVERFLOW); }
                self.y_register += 1;
                if self.y_register < 0 { self.flag_register.set_bit(CPU::NEGATIVE); }
                Ok(true)
            }

            _ => Ok(true)
        }
    }
}