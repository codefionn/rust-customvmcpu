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
    let outfile: Option<String> = None;

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

    let outfile: String = match outfile {
        None => "out.bin".to_string(),
        Some(string) => string
    };

    let mut parser = parser::parse_string(&input);
    let compile_result = compiler::compile(&mut parser);
    if let Some(program) = compile_result {
        if let Result::Ok(_) = fs::write(outfile.clone(), program) {
            println!("Compiled");
        }
        else {
            eprintln!("Could not write to result to {}", outfile);
        }
    }
    else {
        eprintln!("Cannot compile program");
    }
}
