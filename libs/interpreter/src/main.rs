extern crate libcustomvmcpu;

use std::str;
use std::{env, fs, process::exit};
use std::io::{self, Read};

use libcustomvmcpu::{runtime, parser, compiler, common};

fn print_help() {
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        exit(1)
    }
    let args = args.get(1..).expect("Unexpected error");

    let mut registers_to_print: Vec<common::Register> = Vec::new();
    let mut pretty_print_registers = false;
    let mut select = 0;
    while args[select].starts_with("--") {
        match args[select].as_str() {
            "--print-register" => {
                select += 1;
                let register_name = args.get(select).expect("Expected register name");
                registers_to_print.push(match register_name.to_uppercase().as_str() {
                    "RO" => common::Register::R0,
                    "R1" => common::Register::R1,
                    "R2" => common::Register::R2,
                    "R3" => common::Register::R3,
                    "R4" => common::Register::R4,
                    "R5" => common::Register::R5,
                    "R6" => common::Register::R6,
                    "R7" => common::Register::R7,
                    "IP" => common::Register::IP,
                    "SP" => common::Register::SP,
                    "RA" => common::Register::RA,
                    "ERR" => common::Register::ERR,
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
    let input: String = if file != "-" {
        if let Ok(data) = fs::read(file) {
            let result = str::from_utf8(&data[0..]);
            if let Ok(result) = result {
                result.to_string()
            }
            else {
                eprintln!("Error: Could not read from standard input");
                exit(1);
            }
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
        let result = str::from_utf8(&result[0..]);
        if let Ok(result) = result {
            result.to_string()
        }
        else {
            eprintln!("Error: Could not read from standard input");
            exit(1);
        }
    };

    let mut parser = parser::parse_string(&input);
    for error in &parser.errors {
        eprintln!("{:?}, {}", error, input.get(error.pos.clone()).expect("Error in input"));
    }

    let program = compiler::compile(&mut parser);

    if let Some(program) = program {
        let interpreter = runtime::BinaryInterpreter::new_with_initial(&program);
        if let Some(interpreter) = interpreter {
            let mut stdout = std::io::stdout();
            let mut vm = runtime::BinaryVirtualMachine::new(interpreter, &mut stdout);
            let exit_code = vm.execute_first() as i32;

            if pretty_print_registers {
                println!("R0: {}\nR1: {}\nR2: {}\nR3: {}\nR4: {}\nR5: {}\nR6: {}\nR7: {}\nIP: {}\nSP: {}\nRA: {}\nERR: {}\n",
                    vm.read_register_value(common::Register::R0),
                    vm.read_register_value(common::Register::R1),
                    vm.read_register_value(common::Register::R2),
                    vm.read_register_value(common::Register::R3),
                    vm.read_register_value(common::Register::R4),
                    vm.read_register_value(common::Register::R5),
                    vm.read_register_value(common::Register::R6),
                    vm.read_register_value(common::Register::R7),
                    vm.read_register_value(common::Register::IP),
                    vm.read_register_value(common::Register::SP),
                    vm.read_register_value(common::Register::RA),
                    vm.read_register_value(common::Register::ERR),
                );
            }

            exit(exit_code);
        }
        else {
            exit(common::ERROR_START_NUM as i32 + common::Error::Memory as i32);
        }
    }
    else {
        exit(32000)
    }
}
