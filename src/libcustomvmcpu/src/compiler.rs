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

use std::collections::HashMap;
use std::iter::{Filter, Iterator};
use std::mem::size_of;
use std::rc::Rc;
use super::common::{OpCode, Register, Error, LAST_REGISTER, ERROR_START_NUM};
use super::runtime::utils;
use super::parser::{Expr, ParserExpr, ParserResult, ParserError, ParserErrorType, parse_str};

fn filter_errors(program: &mut Vec<ParserExpr>) {
    program.retain(|x| x.expr != Expr::Error());
}

fn calc_expr_size(expr: &Expr) -> u32 {
    return match expr {
        Expr::InstructionTwoRegisters(_, _, _)
            | Expr::InstructionRegisterAndImmediate(_, _, _)
            | Expr::InstructionRegister(_, _)
            | Expr::InstructionImmediate(_, _) => size_of::<u32>() as u32,
        Expr::StoreI32(_) => size_of::<i32>() as u32,
        Expr::StoreStr(string) => string.bytes().len() as u32,
        Expr::Label(_) => 0,
        Expr::Error() => 0,
        _ => {
            panic!("Not a top level expression: {:?}", expr);
        }
    };
}

#[derive(Debug)]
enum CompileExprResult {
    CompileToNone,
    CompileToError,
    CompileToResult(Vec<u8>)
}

struct Compiler<'source> {
    label_map: HashMap<String, u32>,
    parser: &'source mut ParserResult,
}

impl<'source> Compiler<'source> {
    fn interpret_immediate(&mut self, expr: &Expr) -> Option<u32> {
        match expr {
            Expr::Int(result) => Some(*result),
            Expr::AddrToLabel(label) => {
                println!("{:?}", self.label_map);
                if let Some(result) = self.label_map.get(label) {
                    Some(*result)
                }
                else {
                    None
                }
            },
            _ => {
                // No such immediate
                None
            }
        }
    }

    fn compile_expr(&mut self, expr: &ParserExpr, prog_pos: u32) -> CompileExprResult {
        match &expr.expr {
            Expr::Label(label) => {
                self.label_map.insert(label.clone(), prog_pos);
                println!("{:?}", self.label_map);
                CompileExprResult::CompileToNone
            },
            Expr::InstructionTwoRegisters(op_code, reg0, reg1) => {
                CompileExprResult::CompileToResult(utils::create_instruction_two_registers(*op_code, *reg0, *reg1).to_le_bytes().to_vec())
            },
            Expr::InstructionRegister(op_code, reg) => {
                CompileExprResult::CompileToResult(utils::create_instruction_register(*op_code, *reg).to_le_bytes().to_vec())
            },
            Expr::InstructionRegisterAndImmediate(op_code, reg, imm) => {
                if let Some(imm) = self.interpret_immediate(&imm) {
                    CompileExprResult::CompileToResult(utils::create_instruction_register_and_immediate(*op_code, *reg, imm).to_le_bytes().to_vec())
                }
                else {
                    CompileExprResult::CompileToError
                }
            },
            Expr::InstructionImmediate(op_code, imm) => {
                if let Some(imm) = self.interpret_immediate(&imm) {
                    CompileExprResult::CompileToResult(utils::create_instruction_immediate(*op_code, imm).to_le_bytes().to_vec())
                }
                else {
                    CompileExprResult::CompileToError
                }
            },
            Expr::StoreI32(imm) => {
                if let Some(imm) = self.interpret_immediate(&imm) {
                    CompileExprResult::CompileToResult(imm.to_le_bytes().to_vec())
                }
                else {
                    CompileExprResult::CompileToError
                }
            },
            Expr::StoreStr(string) => {
                CompileExprResult::CompileToResult(string.as_bytes().to_vec())
            },
            _ => {
                // Cannot compile expr
                CompileExprResult::CompileToError
            }
        }
    }
}

#[derive(Debug, Clone)]
struct ParserExprWithPos {
    pub pos: u32,
    pub expr: ParserExpr,
}

pub fn compile(parser_result: &mut ParserResult) -> Option<Vec<u8>> {
    let mut program = parser_result.program.clone();
    filter_errors(&mut program);

    let result_size: u32 = program.iter().map(|expr| calc_expr_size(&expr.expr)).sum();

    let mut result: Vec<u8> = vec![0; result_size as usize];
    let mut compiler = Compiler { label_map: HashMap::new(), parser: parser_result };

    let mut filtered_program_with_pos: Vec<ParserExprWithPos> = Vec::with_capacity(program.len());
    let mut work_on_storage_pos: u32 = 0;
    for expr in &program {
        filtered_program_with_pos.push(ParserExprWithPos { pos: work_on_storage_pos, expr: expr.clone() });
        work_on_storage_pos += calc_expr_size(&expr.expr);
    }

    // The following loop tries to reduce filtered_program_with_pos as long as its possible
    let mut old_len = filtered_program_with_pos.len();
    loop {
        filtered_program_with_pos.retain(|expr| {
            let expr_result = compiler.compile_expr(&expr.expr, expr.pos);
            println!("{:?}", expr_result);
            return !(match expr_result {
                    CompileExprResult::CompileToResult(expr_to_bytes) => {
                    println!("{:?}", expr_to_bytes);
                    result.get_mut(expr.pos as usize..(expr.pos as usize + expr_to_bytes.len())).expect("Made sure").copy_from_slice(expr_to_bytes.as_slice());
                    true
                }
                CompileExprResult::CompileToNone => true,
                CompileExprResult::CompileToError => false
            });
        });

        let new_len = filtered_program_with_pos.len();
        if old_len == new_len {
            break; // Cannot reduce (or 0)
        }

        old_len = new_len;
    }

    for expr in filtered_program_with_pos {
        parser_result.errors.push(ParserError { pos: expr.expr.pos.clone(), err_type: ParserErrorType::CannotCompileExpression });
    }


    if !parser_result.errors.is_empty() {
        return None;
    }

    return Some(result);
}

pub fn parse_and_compile_str(program: &'static str) -> Option<Vec<u8>> {
    let mut parser = parse_str(program);
    compile(&mut parser)
}

#[cfg(test)]
mod tests_compiler {
    use super::{compile, parse_and_compile_str, utils, Register, OpCode};

    #[test]
    fn cpy() {
        let result = parse_and_compile_str("cpy $r1, $r4");
        assert_eq!(Some(utils::create_instruction_two_registers(OpCode::CPY, Register::R1, Register::R4).to_le_bytes().to_vec()), result);

        let result = parse_and_compile_str("cpy $r1, $r4\ncpy $r1, $r4");
        assert_eq!(Some(
                [utils::create_instruction_two_registers(OpCode::CPY, Register::R1, Register::R4).to_le_bytes(),
                 utils::create_instruction_two_registers(OpCode::CPY, Register::R1, Register::R4).to_le_bytes()].concat().to_vec()), result);
    }

    #[test]
    fn li() {
        let result = parse_and_compile_str("li $r1, 4");
        assert_eq!(Some(utils::create_instruction_register_and_immediate(OpCode::LI, Register::R1, 4).to_le_bytes().to_vec()), result);
    }

    #[test]
    fn label_standalone() {
        let result = parse_and_compile_str("label:");
        assert_eq!(Some(Vec::new()), result);
    }

    #[test]
    fn label_with_instruction() {
        let result = parse_and_compile_str("label: cpy $r1, $r4");
        assert_eq!(Some(utils::create_instruction_two_registers(OpCode::CPY, Register::R1, Register::R4).to_le_bytes().to_vec()), result);
    }

    #[test]
    fn label_called_upfront() {
        let result = parse_and_compile_str("li $r1, %label\nlabel: cpy $r1, $r4");
        assert_eq!(Some(
                [utils::create_instruction_register_and_immediate(OpCode::LI, Register::R1, 4).to_le_bytes(),
                utils::create_instruction_two_registers(OpCode::CPY, Register::R1, Register::R4).to_le_bytes()].concat().to_vec()), result);
    }
}
