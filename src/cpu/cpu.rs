use std::thread;
use crate::mem::ram::RAM;
use crate::mem::{Byte, DoubleWord, Word};
use crate::ucode::assembly::Assembly;
use crate::ucode::ucode::UCode;

pub struct CPU {
    a_register: Word,
    x_register: Word,
    y_register: Word,

    /// negative, overflow, -, break, decimal, interrupt, zero, carry,
    flag_register: Byte,

    /// instruction step to execute, max 8 steps per instructions
    /// each instruction takes the same amount of time as each instruction has to complete all 8 clock cycles
    instruction_step: u8,

    stack_pointer: DoubleWord,
    program_counter: DoubleWord
}


// FIXME SHIT this does nt add up
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
            instruction_step: 0
        }
    }

    fn execute(&mut self, opcode: u8, ram: &mut RAM) -> Result<bool, UCode> {
        match opcode {
            _ => Ok(false)
        }
    }

    pub fn launch(&mut self, ram: &mut RAM) {
        let mut instruction: Byte = Assembly::HLT;

        loop {
            if self.instruction_step >= 8 {
                while ram.is_locked() {};
                ram.lock().unwrap();
                let res = ram.fetch_byte(self.program_counter as usize);
                match if res.is_err() { self.resolve_error(res.err().unwrap()) } else { UCode::RETRY } {
                    UCode::TERMINATE => { self.terminate() }
                    UCode::RETRY => { continue; }
                    _ => ()
                }

                self.program_counter += 1;
            }

            // TODO use returned bool for premature termination
            self.execute(instruction, ram)
        }
    }

    // TODO
    pub fn resolve_error(ucode: Byte) -> Byte {
        match ucode { _ => { UCode::RETRY } }
    }
}