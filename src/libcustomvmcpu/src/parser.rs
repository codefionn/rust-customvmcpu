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

use super::common::{OpCode, Register};

extern crate logos;
use logos::{Logos, Lexer};
use more_asserts::{assert_ge, debug_assert_ge};

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub enum Token {
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*:")]
    Label,

    #[regex("%[a-zA-Z_][a-zA-Z0-9_]*")]
    AddrToLabel,

    #[regex("\\$[a-z]+[0-9]?")]
    Reg,

    #[regex("0x[A-Z0-9]+")]
    Hex,

    #[regex("[1-9][0-9]*|0")]
    Int,

    #[regex("\"([^\"\\\\]|\\\\.)*\"")]
    String,

    #[token("cpy")]
    KwCpy,

    #[token("lw")]
    KwLw,

    #[token("sw")]
    KwSw,

    #[token("lh")]
    KwLh,

    #[token("sh")]
    KwSh,

    #[token("lb")]
    KwLb,

    #[token("sb")]
    KwSb,

    #[token("li")]
    KwLi,

    #[token("lwi")]
    KwLwI,

    #[token("swi")]
    KwSwI,

    #[token("lhi")]
    KwLhI,

    #[token("shi")]
    KwShI,

    #[token("lbi")]
    KwLbI,

    #[token("sbi")]
    KwSbI,

    #[token("add")]
    KwAdd,

    #[token("sub")]
    KwSub,

    #[token("mul")]
    KwMul,

    #[token("div")]
    KwDiv,

    #[token("addi")]
    KwAddI,

    #[token("subi")]
    KwSubI,

    #[token("muli")]
    KwMulI,

    #[token("divi")]
    KwDivI,

    #[token("and")]
    KwAnd,

    #[token("or")]
    KwOr,

    #[token("xor")]
    KwXor,

    #[token("not")]
    KwNot,

    #[token("j")]
    KwJ,

    #[token("ji")]
    KwJi,

    #[token("jil")]
    KwJil,

    #[token("jzi")]
    KwJzi,

    #[token("jnzi")]
    KwJnzi,

    #[token("jlzi")]
    KwJlzi,

    #[token("jgzi")]
    KwJgzi,

    #[token("syscalli")]
    KwSyscalli,

    #[token("srl")]
    KwSrl,

    #[token("sll")]
    KwSll,

    #[token("srli")]
    KwSrli,

    #[token("slli")]
    KwSlli,

    #[token(".i32")]
    KwMemI32,

    #[token(".str")]
    KwMemStr,

    #[token(",")]
    Comma,

    #[token("+")]
    OpAdd,

    #[token("-")]
    OpSub,

    #[token("*")]
    OpMul,

    #[token("/")]
    OpDiv,

    #[token("(")]
    OpOpenBracket,

    #[token(")")]
    OpCloseBracket,

    #[regex("\n\r?|\r\n?")]
    NewLine,

    #[error]
    #[regex(r"[ \t\v]|//.*", logos::skip)]
    Error,

}

#[derive(Debug, PartialEq)]
pub enum InstructionParseType {
    TwoRegisters,
    RegisterAndImmediate,
    Register,
    Immediate,
    TwoRegistersAndImmediate,
}

pub fn get_instruction_parse_type(op_code: OpCode) -> InstructionParseType {
    match op_code {
        OpCode::CPY
            | OpCode::LW
            | OpCode::SW
            | OpCode::LH
            | OpCode::SH
            | OpCode::LB
            | OpCode::SB 
            | OpCode::ADD
            | OpCode::SUB
            | OpCode::MUL
            | OpCode::DIV
            | OpCode::AND
            | OpCode::OR
            | OpCode::XOR
            | OpCode::SRL
            | OpCode::SLL => InstructionParseType::TwoRegisters,
        OpCode::SRLI
            | OpCode::SLLI
            | OpCode::JZI
            | OpCode::JNZI
            | OpCode::JLZI
            | OpCode::JGZI
            | OpCode::LI
            | OpCode::ADDI
            | OpCode::SUBI
            | OpCode::MULI
            | OpCode::DIVI 
            | OpCode::SWI
            | OpCode::LWI
            | OpCode::SHI
            | OpCode::LHI
            | OpCode::SBI
            | OpCode::LBI => InstructionParseType::RegisterAndImmediate,
        OpCode::NOT
            | OpCode::J => InstructionParseType::Register,
        OpCode::SYSCALLI
            | OpCode::JI
            | OpCode::JIL => InstructionParseType::Immediate,
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    InstructionRegister(OpCode, Register),
    InstructionImmediate(OpCode, Box<ImmediateExpr>),
    InstructionTwoRegisters(OpCode, Register, Register),
    InstructionRegisterAndImmediate(OpCode, Register, Box<ImmediateExpr>),
    StoreI32(Box<ImmediateExpr>),
    StoreStr(String),
    Label(String),
    Error(),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImmediateExpr {
    Int(u32),
    Add(Box<ImmediateExpr>, Box<ImmediateExpr>),
    Sub(Box<ImmediateExpr>, Box<ImmediateExpr>),
    Mul(Box<ImmediateExpr>, Box<ImmediateExpr>),
    Div(Box<ImmediateExpr>, Box<ImmediateExpr>),
    AddrToLabel(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParserErrorType {
    /// Cannot lex expression
    CannotParse,
    /// Expected register token
    ExpectedRegister,
    ExpectedValidRegister,
    ExpectedImmediate,
    ExpectedValidImmediate,
    ExpectedLabel,
    ExpectedNewLine,
    ExpectedToken(&'static Token),
    CannotCompileExpression,
    InvalidEscapeSquence
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParserError {
    pub pos: std::ops::Range<usize>,
    pub err_type: ParserErrorType,
}

#[derive(Debug, Clone)]
pub struct ParserExpr {
    pub pos: std::ops::Range<usize>,
    pub expr: Expr
}

pub struct ParserResult {
    pub program: Vec<ParserExpr>,
    pub errors: Vec<ParserError>
}

struct Parser {
    errors: Vec<ParserError>
}

pub fn parse_str(program: &'static str) -> ParserResult {
    let mut lex = Token::lexer(program);
    let result = parse(&mut lex);
    result
}

pub fn parse_string(program: &String) -> ParserResult {
    let lex = &mut Token::lexer(&program.as_str());
    let result = parse(lex);
    result
}

pub fn parse(lex: &mut Lexer<Token>) -> ParserResult {
    let mut program: Vec<ParserExpr> = Vec::new();
    let mut parser = Parser { errors: Vec::new() };

    let mut tok = lex.next();
    let mut pos = lex.span();
    while let Some(expr) = parser.parse_expr(&mut tok, lex) {
        program.push(expr);

        // Check position to avoid endless loop
        let new_pos = lex.span();
        if new_pos == pos {
            // Hopefully an error occured
            debug_assert_ge!(parser.errors.len(), 0, "At least on parser-error must exist. Current element ({:?}): {}", tok, lex.slice());
            parser.next(&mut tok, lex);
        }

        pos = new_pos;
    }

    return ParserResult { program, errors: parser.errors };
}

/// Combine two ranges, range0 is the lower bound and range1 is the upper bound
fn combine_range<Idx>(range0: std::ops::Range<Idx>, range1: std::ops::Range<Idx>) -> std::ops::Range<Idx> {
    return range0.start..range1.end;
}

impl Parser {
    /// Advance to next token
    fn next<'source>(&mut self, tok: &'source mut Option<Token>, lex: &mut Lexer<Token>) -> &'source mut Option<Token>
    {
        *tok = lex.next();
        println!("{:?}", *tok);
        return tok;
    }

    /// Parse a single expression, like an instruction
    pub fn parse_expr(&mut self, current: &mut Option<Token>, lex: &mut Lexer<Token>) -> Option<ParserExpr>
    {
        println!("{:?}", current);
        self.advance_newlines(current, lex);

        let tok = (*current)?;
         println!("{:?}", tok);
         Some(match tok {
             Token::KwCpy => self.parse_instruction(OpCode::CPY, current, lex),
             Token::KwLw => self.parse_instruction(OpCode::LW, current, lex),
             Token::KwSw => self.parse_instruction(OpCode::SW, current, lex),
             Token::KwLh => self.parse_instruction(OpCode::LH, current, lex),
             Token::KwSh => self.parse_instruction(OpCode::SH, current, lex),
             Token::KwLb => self.parse_instruction(OpCode::LB, current, lex),
             Token::KwSb => self.parse_instruction(OpCode::SB, current, lex),
             Token::KwLi => self.parse_instruction(OpCode::LI, current, lex),
             Token::KwLwI => self.parse_instruction(OpCode::LWI, current, lex),
             Token::KwSwI => self.parse_instruction(OpCode::SWI, current, lex),
             Token::KwLhI => self.parse_instruction(OpCode::LHI, current, lex),
             Token::KwShI => self.parse_instruction(OpCode::SHI, current, lex),
             Token::KwLbI => self.parse_instruction(OpCode::LBI, current, lex),
             Token::KwSbI => self.parse_instruction(OpCode::SBI, current, lex),
             Token::KwAdd => self.parse_instruction(OpCode::ADD, current, lex),
             Token::KwSub => self.parse_instruction(OpCode::SUB, current, lex),
             Token::KwMul => self.parse_instruction(OpCode::MUL, current, lex),
             Token::KwDiv => self.parse_instruction(OpCode::DIV, current, lex),
             Token::KwAddI => self.parse_instruction(OpCode::ADDI, current, lex),
             Token::KwSubI => self.parse_instruction(OpCode::SUBI, current, lex),
             Token::KwMulI => self.parse_instruction(OpCode::MULI, current, lex),
             Token::KwDivI => self.parse_instruction(OpCode::DIVI, current, lex),
             Token::KwAnd => self.parse_instruction(OpCode::AND, current, lex),
             Token::KwOr => self.parse_instruction(OpCode::OR, current, lex),
             Token::KwXor => self.parse_instruction(OpCode::XOR, current, lex),
             Token::KwSrl => self.parse_instruction(OpCode::SRL, current, lex),
             Token::KwSll => self.parse_instruction(OpCode::SLL, current, lex),
             Token::KwSrli => self.parse_instruction(OpCode::SRLI, current, lex),
             Token::KwSlli => self.parse_instruction(OpCode::SLLI, current, lex),
             Token::KwNot => self.parse_instruction(OpCode::NOT, current, lex),
             Token::KwJ => self.parse_instruction(OpCode::J, current, lex),
             Token::KwJi => self.parse_instruction(OpCode::JI, current, lex),
             Token::KwJil => self.parse_instruction(OpCode::JIL, current, lex),
             Token::KwJzi => self.parse_instruction(OpCode::JZI, current, lex),
             Token::KwJnzi => self.parse_instruction(OpCode::JNZI, current, lex),
             Token::KwJlzi => self.parse_instruction(OpCode::JLZI, current, lex),
             Token::KwJgzi => self.parse_instruction(OpCode::JGZI, current, lex),
             Token::KwSyscalli => self.parse_instruction(OpCode::SYSCALLI, current, lex),
             Token::Label => self.parse_label(current, lex),
             Token::AddrToLabel => ParserExpr { pos: lex.span(), expr : Expr::Error() },
             Token::Reg => ParserExpr { pos: lex.span(), expr: Expr::Error() },
             Token::Hex => ParserExpr { pos: lex.span(), expr: Expr::Error() },
             Token::Int => ParserExpr { pos: lex.span(), expr: Expr::Error() },
             Token::String => ParserExpr { pos: lex.span(), expr: Expr::Error() },
             Token::Comma => ParserExpr { pos: lex.span(), expr: Expr::Error() },
             Token::OpAdd => ParserExpr { pos: lex.span(), expr: Expr::Error() },
             Token::OpSub => ParserExpr { pos: lex.span(), expr: Expr::Error() },
             Token::OpMul => ParserExpr { pos: lex.span(), expr: Expr::Error() },
             Token::OpDiv => ParserExpr { pos: lex.span(), expr: Expr::Error() },
             Token::OpOpenBracket => ParserExpr { pos: lex.span(), expr: Expr::Error() },
             Token::OpCloseBracket => ParserExpr { pos: lex.span(), expr: Expr::Error() },
             Token::NewLine => ParserExpr { pos: lex.span(), expr: Expr::Error() },
             Token::Error  => ParserExpr { pos: lex.span(), expr: Expr::Error() },
             Token::KwMemI32 => self.parse_mem_i32(current, lex),
             Token::KwMemStr => self.parse_mem_str(current, lex),
         })
    }

    pub fn parse_mem_str(&mut self, tok: &mut Option<Token>, lex: &mut Lexer<Token>) -> ParserExpr {
        self.next(tok, lex);

        let pos = lex.span();
        let result = if let Some(string) = self.parse_immediate_string(tok, lex) {
            self.expect_newline(tok, lex);
            Expr::StoreStr(string)
        }
        else {
            Expr::Error()
        };

        return ParserExpr { pos, expr: result };
    }

    pub fn parse_immediate_string(&mut self, tok: &mut Option<Token>, lex: &mut Lexer<Token>) -> Option<String> {
        let pos = lex.span();
        eprintln!("Expect: {:?}", *tok);

        if let Some(Token::String) = tok {
            let tokstr = lex.slice();
            let tokstr = tokstr.get(1..(tokstr.len() - 1)).expect("Made sure by lexer").to_string();

            let mut result = String::with_capacity(tokstr.len());
            let mut i = 0;
            while i < tokstr.len() {
                let c: char = tokstr.chars().nth(i).unwrap();
                if c == '\\' {
                    // Escape sequence
                    i += 1;
                    let c: char = tokstr.chars().nth(i).unwrap();
                    let c = match c {
                        'n' => '\n',
                        'r' => '\r',
                        't' => '\t',
                        '0' => '\0',
                        '"' => '"',
                        '\'' => '\'',
                        _ => {
                            self.errors.push(ParserError { pos: pos.start+i..pos.start+i, err_type: ParserErrorType::InvalidEscapeSquence });
                            '?'
                        }
                    };

                    result += c.to_string().as_str();
                }
                else {
                    result += c.to_string().as_str();
                }

                i += 1;

                eprintln!("String: {}", result);
            }

            self.next(tok, lex);
            Some(result.to_string())
        }
        else {
            None
        }
    }

    pub fn parse_mem_i32(&mut self, tok: &mut Option<Token>, lex: &mut Lexer<Token>) -> ParserExpr {
        self.next(tok, lex);

        let pos = lex.span();
        let result = if let Some(expr) = self.parse_immediate(tok, lex) {
            self.expect_newline(tok, lex);
            Expr::StoreI32(Box::new(expr))
        }
        else {
            Expr::Error()
        };

        return ParserExpr { pos, expr: result };
    }

    pub fn parse_label(&mut self, tok: &mut Option<Token>, lex: &mut Lexer<Token>) -> ParserExpr {
        let pos = lex.span();
        if let Some(Token::Label) = tok {
            let result = Expr::Label(lex.slice().get(0..(lex.slice().len() - 1)).expect("Made sure by lexer").to_string());
            self.next(tok, lex);
            ParserExpr { pos, expr: result }
        }
        else {
            self.errors.push(ParserError { pos: lex.span(), err_type: ParserErrorType::ExpectedLabel });
            ParserExpr { pos, expr: Expr::Error() }
        }
    }
    
    pub fn parse_instruction(&mut self, op_code: OpCode, tok: &mut Option<Token>, lex: &mut Lexer<Token>) -> ParserExpr {
        let start = lex.span();

        let parse_type = get_instruction_parse_type(op_code);
        println!("{:?}, {:?}", op_code, parse_type);
        let expr = match parse_type {
            InstructionParseType::Register => {
                self.next(tok, lex);
                let end = lex.span();
                if let Some(reg) = self.parse_register(tok, lex) {
                    ParserExpr { pos: combine_range(start.clone(), end), expr: Expr::InstructionRegister(op_code, reg) }
                }
                else {
                    ParserExpr { pos: combine_range(start.clone(), end), expr: Expr::Error() }
                }
            },
            InstructionParseType::Immediate => {
                self.next(tok, lex);
                let end = lex.span();
                if let Some(imm) = self.parse_immediate(tok, lex) {
                    ParserExpr { pos: combine_range(start.clone(), end), expr: Expr::InstructionImmediate(op_code, Box::new(imm)) }
                }
                else {
                    ParserExpr { pos: combine_range(start.clone(), end), expr: Expr::Error() }
                }
            },
            InstructionParseType::TwoRegisters => {
                self.next(tok, lex);
                let reg_raw0 = self.parse_register(tok, lex);
                self.eat_token(tok, lex, &Token::Comma);
                let reg_raw1 = self.parse_register(tok, lex);

                let end = lex.span();
                if let (Some(reg0), Some(reg1)) = (reg_raw0, reg_raw1) {
                    ParserExpr { pos: combine_range(start.clone(), end), expr: Expr::InstructionTwoRegisters(op_code, reg0, reg1) }
                }
                else {
                    ParserExpr { pos: combine_range(start.clone(), end), expr: Expr::Error() }
                }
            },
            InstructionParseType::RegisterAndImmediate => {
                self.next(tok, lex);
                let reg_raw = self.parse_register(tok, lex);
                self.eat_token(tok, lex, &Token::Comma);
                let imm_raw = self.parse_immediate(tok, lex);

                let end = lex.span();
                if let (Some(reg), Some(imm)) = (reg_raw, imm_raw) {
                    ParserExpr { pos: combine_range(start.clone(), end), expr: Expr::InstructionRegisterAndImmediate(op_code, reg, Box::new(imm)) }
                }
                else {
                    ParserExpr { pos: combine_range(start.clone(), end), expr: Expr::Error() }
                }
            },
            InstructionParseType::TwoRegistersAndImmediate => {
                ParserExpr { pos: start.clone(), expr: Expr::Error() }
            }
        };

        if !self.expect_newline(tok, lex) {
            return ParserExpr { pos: combine_range(start.clone(), lex.span()), expr: Expr::Error() };
        }

        return expr;
    }

    fn expect_token(&mut self, tok: &mut Option<Token>, lex: &mut Lexer<Token>, expect: &'static Token) -> bool {
        if *tok != Some(*expect) {
            self.errors.push(ParserError { pos: lex.span(), err_type: ParserErrorType::ExpectedToken(expect) });
            return false;
        }

        return true;
    }

    fn eat_token(&mut self, tok: &mut Option<Token>, lex: &mut Lexer<Token>, expect: &'static Token) -> bool {
        if self.expect_token(tok, lex, expect) {
            self.next(tok, lex);

            return true;
        }

        return false;
    }

    fn expect_newline(&mut self, tok: &mut Option<Token>, lex: &mut Lexer<Token>) -> bool {
        if !self.advance_newlines(tok, lex) {
            eprintln!("Expected newline, not: {:?}", tok);
            self.errors.push(ParserError { pos: lex.span(), err_type: ParserErrorType::ExpectedValidImmediate });
            return false;
        }

        return true;
    }

    fn advance_newlines(&mut self, tok: &mut Option<Token>, lex: &mut Lexer<Token>) -> bool {
        if None == *tok {
            return true;
        }

        let mut result = false;
        while *tok == Some(Token::NewLine) {
            self.next(tok, lex);
            result = true;
        }
        return result;
    }

    fn parse_immediate(&mut self, current: &mut Option<Token>, lex: &mut Lexer<Token>) -> Option<ImmediateExpr> {
        if let Some(tok) = current {
            match tok {
                Token::Int => {
                    let result = Some(ImmediateExpr::Int(lex.slice().parse().expect("Expect rangers everything was made sure!")));
                    self.next(current, lex); // eat int
                    result
                },
                Token::AddrToLabel => {
                    let result = Some(ImmediateExpr::AddrToLabel(lex.slice().get(1..).expect("Made sure by lexer").into()));
                    self.next(current, lex); // eat addr_to_label
                    result
                },
                _ => {
                    self.errors.push(ParserError { pos: lex.span(), err_type: ParserErrorType::ExpectedValidImmediate });
                    None
                }
            }
        }
        else {
            self.errors.push(ParserError { pos: lex.span(), err_type: ParserErrorType::ExpectedImmediate });
            None
        }
    }

    fn parse_register(&mut self, tok: &mut Option<Token>, lex: &mut Lexer<Token>) -> Option<Register> {
        return if let Some(Token::Reg) = *tok {
            let reg = lex.slice().get(1..).expect("It starts with $, damit!");
            let result = match reg {
                "r0" => Some(Register::R0),
                "r1" => Some(Register::R1),
                "r2" => Some(Register::R2),
                "r3" => Some(Register::R3),
                "r4" => Some(Register::R4),
                "r5" => Some(Register::R5),
                "r6" => Some(Register::R6),
                "r7" => Some(Register::R7),
                "ip" => Some(Register::IP),
                "ra" => Some(Register::RA),
                "sp" => Some(Register::SP),
                "err" => Some(Register::ERR),
                _ => {
                    eprintln!("Expected register: {:?}", tok);
                    self.errors.push(ParserError { pos: lex.span(), err_type: ParserErrorType::ExpectedValidRegister });
                    None
                }
            };

            if result != None {
                self.next(tok, lex); // eat register token
            }

            result
        }
        else {
            self.errors.push(ParserError { pos: lex.span(), err_type: ParserErrorType::ExpectedRegister });
            None
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::common::{OpCode, Register};
    use super::{Token, parse_str, parse_string, ParserResult, Expr, ImmediateExpr};
    use logos::{Logos, Lexer};

    #[test]
    fn newline() {
        let mut lex = Token::lexer("\n");
        assert_eq!(Some(Token::NewLine), lex.next());
        assert_eq!(None, lex.next());

        let mut lex = Token::lexer("\n\r");
        assert_eq!(Some(Token::NewLine), lex.next());
        assert_eq!(None, lex.next());

        let mut lex = Token::lexer("\r\n");
        assert_eq!(Some(Token::NewLine), lex.next());
        assert_eq!(None, lex.next());

        let mut lex = Token::lexer("\r");
        assert_eq!(Some(Token::NewLine), lex.next());
        assert_eq!(None, lex.next());

        let mut lex = Token::lexer("//Hello world\n");
        assert_eq!(Some(Token::NewLine), lex.next());
        assert_eq!(None, lex.next());

        let mut lex = Token::lexer("//Hello world\n//Hello, world\n");
        assert_eq!(Some(Token::NewLine), lex.next());
        assert_eq!(Some(Token::NewLine), lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn label() {
        let mut lex = Token::lexer("label:");
        assert_eq!(Some(Token::Label), lex.next());
    }

    #[test]
    fn addr_to_label() {
        let mut lex = Token::lexer("%label");
        assert_eq!(Some(Token::AddrToLabel), lex.next());
    }

    #[test]
    fn register() {
        let mut lex = Token::lexer("$r0");
        assert_eq!(Some(Token::Reg), lex.next());
    }

    #[test]
    fn lw() {
        let mut lex = Token::lexer("lw");
        assert_eq!(Some(Token::KwLw), lex.next());
    }

    #[test]
    fn sw() {
        let mut lex = Token::lexer("sw");
        assert_eq!(Some(Token::KwSw), lex.next());
    }

    #[test]
    fn lh() {
        let mut lex = Token::lexer("lh");
        assert_eq!(Some(Token::KwLh), lex.next());
    }

    #[test]
    fn sh() {
        let mut lex = Token::lexer("sh");
        assert_eq!(Some(Token::KwSh), lex.next());
    }

    #[test]
    fn lb() {
        let mut lex = Token::lexer("lb");
        assert_eq!(Some(Token::KwLb), lex.next());
    }

    #[test]
    fn sb() {
        let mut lex = Token::lexer("sb");
        assert_eq!(Some(Token::KwSb), lex.next());
    }

    #[test]
    fn li() {
        let mut lex = Token::lexer("li");
        assert_eq!(Some(Token::KwLi), lex.next());
    }

    #[test]
    fn add() {
        let mut lex = Token::lexer("add");
        assert_eq!(Some(Token::KwAdd), lex.next());
    }

    #[test]
    fn sub() {
        let mut lex = Token::lexer("sub");
        assert_eq!(Some(Token::KwSub), lex.next());
    }

    #[test]
    fn mul() {
        let mut lex = Token::lexer("mul");
        assert_eq!(Some(Token::KwMul), lex.next());
    }

    #[test]
    fn div() {
        let mut lex = Token::lexer("div");
        assert_eq!(Some(Token::KwDiv), lex.next());
    }

    #[test]
    fn and() {
        let mut lex = Token::lexer("and");
        assert_eq!(Some(Token::KwAnd), lex.next());
    }

    #[test]
    fn or() {
        let mut lex = Token::lexer("or");
        assert_eq!(Some(Token::KwOr), lex.next());
    }

    #[test]
    fn xor() {
        let mut lex = Token::lexer("xor");
        assert_eq!(Some(Token::KwXor), lex.next());
    }

    #[test]
    fn srl() {
        let mut lex = Token::lexer("srl");
        assert_eq!(Some(Token::KwSrl), lex.next());
    }

    #[test]
    fn sll() {
        let mut lex = Token::lexer("sll");
        assert_eq!(Some(Token::KwSll), lex.next());
    }

    #[test]
    fn srli() {
        let mut lex = Token::lexer("srli");
        assert_eq!(Some(Token::KwSrli), lex.next());
    }

    #[test]
    fn slli() {
        let mut lex = Token::lexer("slli");
        assert_eq!(Some(Token::KwSlli), lex.next());
    }

    #[test]
    fn not() {
        let mut lex = Token::lexer("not");
        assert_eq!(Some(Token::KwNot), lex.next());
    }

    #[test]
    fn j() {
        let mut lex = Token::lexer("j");
        assert_eq!(Some(Token::KwJ), lex.next());
    }

    #[test]
    fn ji() {
        let mut lex = Token::lexer("ji");
        assert_eq!(Some(Token::KwJi), lex.next());
    }

    #[test]
    fn jil() {
        let mut lex = Token::lexer("jil");
        assert_eq!(Some(Token::KwJil), lex.next());
    }

    #[test]
    fn jzi() {
        let mut lex = Token::lexer("jzi");
        assert_eq!(Some(Token::KwJzi), lex.next());
    }

    #[test]
    fn jnzi() {
        let mut lex = Token::lexer("jnzi");
        assert_eq!(Some(Token::KwJnzi), lex.next());
    }

    #[test]
    fn jlzi() {
        let mut lex = Token::lexer("jlzi");
        assert_eq!(Some(Token::KwJlzi), lex.next());
    }

    #[test]
    fn jgzi() {
        let mut lex = Token::lexer("jgzi");
        assert_eq!(Some(Token::KwJgzi), lex.next());
    }

    #[test]
    fn syscalli() {
        let mut lex = Token::lexer("syscalli");
        assert_eq!(Some(Token::KwSyscalli), lex.next());
    }

    #[test]
    fn int() {
        let mut lex = Token::lexer("0");
        assert_eq!(Some(Token::Int), lex.next());

        let mut lex = Token::lexer("1");
        assert_eq!(Some(Token::Int), lex.next());

        let mut lex = Token::lexer("10");
        assert_eq!(Some(Token::Int), lex.next());

        let mut lex = Token::lexer("9");
        assert_eq!(Some(Token::Int), lex.next());

        let mut lex = Token::lexer("8");
        assert_eq!(Some(Token::Int), lex.next());

        let mut lex = Token::lexer("98");
        assert_eq!(Some(Token::Int), lex.next());

        let mut lex = Token::lexer("9876543210");
        assert_eq!(Some(Token::Int), lex.next());

        let mut lex = Token::lexer("1230");
        assert_eq!(Some(Token::Int), lex.next());
    }

    #[test]
    fn hex() {
        let mut lex = Token::lexer("0x0");
        assert_eq!(Some(Token::Hex), lex.next());

        let mut lex = Token::lexer("0x10");
        assert_eq!(Some(Token::Hex), lex.next());

        let mut lex = Token::lexer("0x01");
        assert_eq!(Some(Token::Hex), lex.next());

        let mut lex = Token::lexer("0xFF");
        assert_eq!(Some(Token::Hex), lex.next());

        let mut lex = Token::lexer("0x1F");
        assert_eq!(Some(Token::Hex), lex.next());

        let mut lex = Token::lexer("0xFA");
        assert_eq!(Some(Token::Hex), lex.next());

        let mut lex = Token::lexer("0xABCDEF");
        assert_eq!(Some(Token::Hex), lex.next());

        let mut lex = Token::lexer("0x0123456789");
        assert_eq!(Some(Token::Hex), lex.next());
    }

    #[test]
    fn parse_li() {
        let result = parse_str("li $r0, 10");
        println!("{:?}", result.program);
        println!("{:?}", result.errors);
        assert_eq!(1, result.program.len());
        let expr = result.program.get(0).expect("Made sure above");
        assert_eq!(Expr::InstructionRegisterAndImmediate(OpCode::LI, Register::R0, Box::new(ImmediateExpr::Int(10))), expr.expr);
    }

    #[test]
    fn parse_j() {
        let result = parse_str("j $r0");
        assert_eq!(1, result.program.len());
        let expr = result.program.get(0).expect("Made sure above");
        assert_eq!(Expr::InstructionRegister(OpCode::J, Register::R0), expr.expr);
    }

    #[test]
    fn parse_add() {
        let result = parse_str("add $r0, $r1");
        assert_eq!(1, result.program.len());
        let expr = result.program.get(0).expect("Made sure above");
        assert_eq!(Expr::InstructionTwoRegisters(OpCode::ADD, Register::R0, Register::R1), expr.expr);
    }

    #[test]
    fn parse_addi() {
        let result = parse_str("addi $r0, 11");
        assert_eq!(1, result.program.len());
        let expr = result.program.get(0).expect("Made sure above");
        assert_eq!(Expr::InstructionRegisterAndImmediate(OpCode::ADDI, Register::R0, Box::new(ImmediateExpr::Int(11))), expr.expr);
    }

    #[test]
    fn parse_subi() {
        let result = parse_str("subi $r0, 11");
        assert_eq!(1, result.program.len());
        let expr = result.program.get(0).expect("Made sure above");
        assert_eq!(Expr::InstructionRegisterAndImmediate(OpCode::SUBI, Register::R0, Box::new(ImmediateExpr::Int(11))), expr.expr);
    }

    #[test]
    fn parse_muli() {
        let result = parse_str("muli $r0, 11");
        assert_eq!(1, result.program.len());
        let expr = result.program.get(0).expect("Made sure above");
        assert_eq!(Expr::InstructionRegisterAndImmediate(OpCode::MULI, Register::R0, Box::new(ImmediateExpr::Int(11))), expr.expr);
    }

    #[test]
    fn parse_divi() {
        let result = parse_str("divi $r0, 11");
        assert_eq!(1, result.program.len());
        let expr = result.program.get(0).expect("Made sure above");
        assert_eq!(Expr::InstructionRegisterAndImmediate(OpCode::DIVI, Register::R0, Box::new(ImmediateExpr::Int(11))), expr.expr);
    }

    #[test]
    fn parse_jgzi() {
        let result = parse_str("jgzi $r0, 10");
        assert_eq!(1, result.program.len());
        let expr = result.program.get(0).expect("Made sure above");
        assert_eq!(Expr::InstructionRegisterAndImmediate(OpCode::JGZI, Register::R0, Box::new(ImmediateExpr::Int(10))), expr.expr);

        let result = parse_str("jgzi $r0, %label");
        assert_eq!(1, result.program.len());
        let expr = result.program.get(0).expect("Made sure above");
        assert_eq!(Expr::InstructionRegisterAndImmediate(OpCode::JGZI, Register::R0, Box::new(ImmediateExpr::AddrToLabel("label".to_string()))), expr.expr);
    }

    #[test]
    fn parse_mem_i32() {
        let result = parse_str(".i32 13");
        assert_eq!(1, result.program.len());
        let expr = result.program.get(0).expect("Made sure above");
        assert_eq!(Expr::StoreI32(Box::new(ImmediateExpr::Int(13))), expr.expr);

        let result = parse_str(".i32 13\n.i32 9");
        assert_eq!(2, result.program.len());
        let expr = result.program.get(0).expect("Made sure above");
        assert_eq!(Expr::StoreI32(Box::new(ImmediateExpr::Int(13))), expr.expr);
        let expr = result.program.get(1).expect("Made sure above");
        assert_eq!(Expr::StoreI32(Box::new(ImmediateExpr::Int(9))), expr.expr);
    }

    #[test]
    fn parse_mem_str() {
        let result = parse_str(".str \"Hello, world!\"");
        assert_eq!(1, result.program.len());
        let expr = result.program.get(0).expect("Made sure above");
        assert_eq!(Expr::StoreStr("Hello, world!".to_string()), expr.expr);
    }

    #[test]
    fn parse_instructions_two_registers() {
        let op_codes = [ OpCode::CPY,
            OpCode::LW, OpCode::SW,
            OpCode::LH, OpCode::SH,
            OpCode::LB, OpCode::SB,
            OpCode::ADD, OpCode::SUB, OpCode::MUL, OpCode::DIV,
            OpCode::AND, OpCode::OR, OpCode::XOR,
            OpCode::SRL, OpCode::SLL ];

        for op_code in op_codes {
            let result = parse_string(&(op_code.to_string() + " $r0, $ra"));
            assert_eq!(1, result.program.len());
            let expr = result.program.get(0).expect("Made sure above");
            assert_eq!(Expr::InstructionTwoRegisters(op_code, Register::R0, Register::RA), expr.expr);
        }
    }

    #[test]
    fn parse_instructions_register_and_immediate() {
        let op_codes = [ OpCode::SRLI,
            OpCode::SLLI,
            OpCode::JZI,
            OpCode::JNZI,
            OpCode::JLZI,
            OpCode::JGZI,
            OpCode::LI ];

        for op_code in op_codes {
            let result = parse_string(&(op_code.to_string() + " $r0, 10"));
            assert_eq!(1, result.program.len());
            let expr = result.program.get(0).expect("Made sure above");
            assert_eq!(Expr::InstructionRegisterAndImmediate(op_code, Register::R0, Box::new(ImmediateExpr::Int(10))), expr.expr);
        }
    }

    #[test]
    fn parse_instruction_register() {
        let op_codes = [ OpCode::NOT, OpCode::J ];

        for op_code in op_codes {
            let result = parse_string(&(op_code.to_string() + " $r6"));
            assert_eq!(1, result.program.len());
            let expr = result.program.get(0).expect("Made sure above");
            assert_eq!(Expr::InstructionRegister(op_code, Register::R6), expr.expr);
        }
    }

    #[test]
    fn parse_instruction_immediate() {
        let op_codes = [ OpCode::SYSCALLI, OpCode::JI, OpCode::JIL ];

        for op_code in op_codes {
            let result = parse_string(&(op_code.to_string() + " 102"));
            assert_eq!(1, result.program.len());
            let expr = result.program.get(0).expect("Made sure above");
            assert_eq!(Expr::InstructionImmediate(op_code, Box::new(ImmediateExpr::Int(102))), expr.expr);
        }
    }
}
