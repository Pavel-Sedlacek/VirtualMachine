use std::path::Display;
use std::sync::Mutex;
use std::thread;

use crate::lib::bus::Bus;
use crate::lib::cpu::cpu::CPU;
use crate::lib::gpu::gpu::GPU;
use crate::lib::gpu::monitor::Monitor;
use crate::lib::mem::ram::RAM;
use crate::lib::ucode::cpu_assembly::CPUAssembly;

pub mod lib;

fn main() {
    // 536870912 * 8 => 4 GB => 4096 MB
    // address range => 0x0000'0000 <-> 0x1FFF'FFFF
    let mut ram = RAM::new(536_870_912);
    let mut bus = Mutex::new(Bus::new());

    let mut m1 = Monitor::new();

    let mut cpu = CPU::new();
    let mut gpu = GPU::new();

    let cpu_thread = thread::spawn(move || {
        cpu.launch(&mut ram, &mut bus)
    });

    thread::spawn(move || {
        gpu.launch(&mut bus, &[&mut m1])
    });

    while !cpu_thread.is_finished() {}
}