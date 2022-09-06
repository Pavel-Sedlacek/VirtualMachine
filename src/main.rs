use std::thread;

use crate::lib::cpu::cpu::CPU;
use crate::lib::mem::ram::RAM;
use crate::lib::ucode::assembly::Assembly;

pub mod lib;

fn main() {

    // 4294967296 * 8 ~ 32GB
    // 0x0000'0000 <-> 0xffff'ffff
    let mut ram = RAM::new(4_294_967_296);

    ram.write_byte(0xffff_fff0, Assembly::LDA);
    ram.write_word(0xffff_fff1, 60000);

    let mut cpu = CPU::new();

    let x = thread::spawn(move || {
        cpu.launch(&mut ram)
    });

    // thread::spawn(move || {
    //     lib.cpu.launch(&mut ram)
    // });
    while !x.is_finished() {}
}
