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

mod runtime;

use runtime::{Interpreter, BinaryVirtualMachine, OpCode, BinaryInterpreter};

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
    let input = fs::read(file);
    match input {
        Ok(data) => {
            let interpreter = BinaryInterpreter::new_with_initial(&data);
            let vm = BinaryVirtualMachine::new(interpreter);
            exit(vm.execute_first() as i32);
        },
        _ => {
            eprintln!("Error: Could not read file \"{}\"", file);
            exit(1);
        }
    }
}
