use std::path::Display;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::lib::bus::bus::Bus;
use crate::lib::cpu::cpu::CPU;
use crate::lib::gpu::gpu::GPU;
use crate::lib::gpu::monitor::Monitor;
use crate::lib::mem::ram::RAM;

pub mod lib;

fn main() {
    // 536870912 * 8 => 4 GB => 4096 MB
    // address range => 0x0000'0000 <-> 0x1FFF'FFFF
    let mut ram = RAM::new(536_870_912);
    let mut bus = Arc::new(Mutex::new(Bus::new()));
    let mut b1 = Arc::clone(&bus);
    let mut b2 = Arc::clone(&bus);

    let mut m1 = Monitor::new(640, 480);
    let mut cpu = CPU::new();

    let mut gpu = GPU::new("vGPU - GACUM (Graphical Accelerated Compute Unit Magic)", "vgpu-acum-0000-0000");

    thread::spawn(move || {
        gpu.launch(&b2, &[&mut m1])
    });

    thread::sleep(Duration::new(0, 50000));

    let cpu_thread = thread::spawn(move || {
        cpu.launch(&mut ram, &b1)
    });

    while !cpu_thread.is_finished() {}
}