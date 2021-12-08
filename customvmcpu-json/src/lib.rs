extern crate libcustomvmcpu;

#[macro_use]
extern crate json;

use libcustomvmcpu::{common, parser, compiler, runtime};

pub fn interpreter_to_json_string(program: &String) -> String {
    interpreter_to_json(program).dump()
}

pub fn interpreter_to_json(program: &String) -> json::JsonValue {
    let mut parser = parser::parse_string(&program);
    let errors_json = json::JsonValue::Array(
        (&parser.errors).iter().map(|error| {
            return object!{
                "pos_start" => error.pos.start,
                "pos_end" => error.pos.end,
                "error_type" => error.err_type.to_string()
            };
    }).collect());

    let program = compiler::compile(&mut parser);

    if let Some(program) = program {
        let interpreter = runtime::BinaryInterpreter::new_with_initial(&program);
        if let Some(interpreter) = interpreter {
            let mut stdout = Vec::new();
            let mut vm = runtime::BinaryVirtualMachine::new(interpreter, &mut stdout);
            let exit_code = vm.execute_first() as i32;

            let registers = object!{
                "R0" => vm.read_register_value(common::Register::R0),
                "R1" => vm.read_register_value(common::Register::R1),
                "R2" => vm.read_register_value(common::Register::R2),
                "R3" => vm.read_register_value(common::Register::R3),
                "R4" => vm.read_register_value(common::Register::R4),
                "R5" => vm.read_register_value(common::Register::R5),
                "R6" => vm.read_register_value(common::Register::R6),
                "R7" => vm.read_register_value(common::Register::R7),
                "IP" => vm.read_register_value(common::Register::IP),
                "SP" => vm.read_register_value(common::Register::SP),
                "RA" => vm.read_register_value(common::Register::RA),
                "ERR" => vm.read_register_value(common::Register::ERR),
            };

            return object!{
                "success" => true,
                "errors" => errors_json,
                "exit_code" => exit_code,
                "stdout" => String::from_utf8(stdout).unwrap_or(String::new()),
                "registers" => registers,
            };
        }
        else {
            return object!{
                "success" => false,
                "errors" => errors_json
            };
        }
    }
    else {
        return object!{
            "success" => false,
            "errors" => errors_json
        };
    }
}

#[cfg(test)]
mod tests {
    use super::{runtime, interpreter_to_json};
    #[test]
    fn basic() {
        let result = interpreter_to_json(&"syscalli 0".into());
        let expect: json::JsonValue = object!{
            "success" => true,
            "errors" => array![],
            "exit_code" => 0,
            "stdout" => String::new(),
            "registers" => object!{
                "R0" => 0,
                "R1" => 0,
                "R2" => 0,
                "R3" => 0,
                "R4" => 0,
                "R5" => 0,
                "R6" => 0,
                "R7" => 0,
                "IP" => 0,
                "SP" => runtime::BINARY_INTERPRETER_MEM_SIZE,
                "RA" => 4,
                "ERR" => 0,
            }
        };

        assert_eq!(
            expect,
            result
        );
    }
}
