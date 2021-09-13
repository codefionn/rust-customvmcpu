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

use libcustomvmcpu::runtime::{Interpreter, BinaryVirtualMachine, OpCode, BinaryInterpreter};

fn print_help() {
    println!("rust-customvmcpu - Virtual CPU written in rust");
    println!("");
    println!("Usage: rust-customvmcpu [Options] <program>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        print_help();
        exit(1);
    }

    let file = args.last().expect("Unknown error"); // Check above: not empty
    let input: Vec<u8> = if file == "-" {
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
    exit(vm.execute_first() as i32);
}
