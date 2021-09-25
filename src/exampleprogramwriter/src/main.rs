extern crate libcustomvmcpu;

use libcustomvmcpu::common::{Register, OpCode};
use libcustomvmcpu::runtime::utils;

use std::fs;

fn write_to_file(path: &str, program: &[u32]) {
    let program_bytes: Vec<u8> = program.iter().flat_map(|x| x.to_le_bytes().to_vec()).collect();
    if let Err(_) = fs::write(path, program_bytes) {
        eprintln!("Could not write to file {}", path);
    }
}

fn write_syscall_exit(path: &str) {
    const PROGRAM: [u32; 1] = [
        utils::create_instruction_immediate(OpCode::SYSCALLI, 0)
    ];

    write_to_file(path, &PROGRAM);
}

fn write_add(path: &str) {
    const PROGRAM: [u32; 4] = [
        utils::create_instruction_register_and_immediate(OpCode::LI, Register::R2, 100),
        utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 32),
        utils::create_instruction_two_registers(OpCode::ADD, Register::R0, Register::R2),
        utils::create_instruction_immediate(OpCode::SYSCALLI, 0)
    ];

    write_to_file(path, &PROGRAM);
}

fn main() {
    if let Err(_) = fs::create_dir("out") {
        fs::metadata("out").expect("Directory out must exist");
    }
    write_syscall_exit("out/syscall_exit.bin");
    write_add("out/add_32_100.bin");
}
