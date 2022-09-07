use std::thread;

use crate::lib::cpu::cpu::CPU;
use crate::lib::mem::ram::RAM;
use crate::lib::ucode::assembly::Assembly;

pub mod lib;

fn main() {
    // 536870912 * 8 => 4 GB => 4096 MB
    // address range => 0x0000'0000 <-> 0x1FFF'FFFF
    let mut ram = RAM::new(536_870_912);

    let mut cpu = CPU::new();

    let cpu_thread = thread::spawn(move || {
        cpu.launch(&mut ram)
    });

    // thread::spawn(move || {
    //     lib.cpu.launch(&mut ram)
    // });
    while !cpu_thread.is_finished() {}
}