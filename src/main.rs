/*
 * 
 * Custom, virtual CPU environment written in Rust
 * Copyright (C) 2021  Fionn Langhans

 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.

 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.

 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::{env, fs, process::exit};
use std::io::{self, Read};

use libcustomvmcpu::runtime::{Interpreter, BinaryVirtualMachine, OpCode, BinaryInterpreter, Register};

fn print_help() {
    println!("rust-customvmcpu - Virtual CPU written in rust");
    println!("");
    println!("Usage: rust-customvmcpu [Options] <program>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        exit(1)
    }
    let args = args.get(1..).expect("Unexpected error");

    let mut registers_to_print: Vec<Register> = Vec::new();
    let mut pretty_print_registers = false;
    let mut select = 0;
    while args[select].starts_with("--") {
        match args[select].as_str() {
            "--print-register" => {
                select += 1;
                let register_name = args.get(select).expect("Expected register name");
                registers_to_print.push(match register_name.to_uppercase().as_str() {
                    "RO" => Register::R0,
                    "R1" => Register::R1,
                    "R2" => Register::R2,
                    "R3" => Register::R3,
                    "R4" => Register::R4,
                    "R5" => Register::R5,
                    "R6" => Register::R6,
                    "R7" => Register::R7,
                    "IP" => Register::IP,
                    "SP" => Register::SP,
                    "RA" => Register::RA,
                    "ERR" => Register::ERR,
                    _ => {
                        eprintln!("Expected register name");
                        exit(1)
                    }
                })
            },
            "--register-table" => {
                pretty_print_registers = true;
            },
            _ => {
                eprintln!("Unknown Option: {}", args[select]);
                exit(1);
            }
        }

        select += 1;
    }

    let file = args.last().expect("Expected filepath"); // Check above: not empty
    let input: Vec<u8> = if file != "-" {
        if let Ok(data) = fs::read(file) {
            data
        }
        else {
            eprintln!("Error: Could not read file \"{}\"", file);
            exit(1);
        }
    } else {
        let mut result: Vec<u8> = Vec::new();
        if let Err(_) = io::stdin().lock().read_to_end(&mut result) {
            eprintln!("Error: Could not read from standard input");
            exit(1);
        }
        result
    };

    let interpreter = BinaryInterpreter::new_with_initial(&input);
    let mut vm = BinaryVirtualMachine::new(interpreter);
    let exit_code = vm.execute_first() as i32;

    if pretty_print_registers {
        println!("R0: {}\nR1: {}\nR2: {}\nR3: {}\nR4: {}\nR5: {}\nR6: {}\nR7: {}\nIP: {}\nSP: {}\nRA: {}\nERR: {}\n",
            vm.read_register_value(Register::R0),
            vm.read_register_value(Register::R1),
            vm.read_register_value(Register::R2),
            vm.read_register_value(Register::R3),
            vm.read_register_value(Register::R4),
            vm.read_register_value(Register::R5),
            vm.read_register_value(Register::R6),
            vm.read_register_value(Register::R7),
            vm.read_register_value(Register::IP),
            vm.read_register_value(Register::SP),
            vm.read_register_value(Register::RA),
            vm.read_register_value(Register::ERR),
        );
    }

    exit(exit_code);
}
