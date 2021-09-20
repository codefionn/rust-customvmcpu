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

use super::common::{OpCode, Register, Error, LAST_REGISTER, ERROR_START_NUM};
use super::runtime::utils;

extern crate logos;
use logos::{Logos, Lexer};

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

    #[token("add")]
    KwAdd,

    #[token("sub")]
    KwSub,

    #[token("mul")]
    KwMul,

    #[token("div")]
    KwDiv,

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

pub enum InstructionParseType {
    TwoRegisters,
    RegisterAndImmediate,
    Register,
    Immediate,
    TwoRegistersAndImmediate,
}

pub fn get_op_code(tok: Token) -> Option<OpCode> {
    match tok {
        Token::KwCpy => Some(OpCode::CPY),
        Token::KwLw => Some(OpCode::LW),
        Token::KwSw => Some(OpCode::SW),
        Token::KwLh => Some(OpCode::LH),
        Token::KwSh => Some(OpCode::SH),
        Token::KwLb => Some(OpCode::LB),
        Token::KwSb => Some(OpCode::SB),
        Token::KwLi => Some(OpCode::LI),
        Token::KwAdd => Some(OpCode::ADD),
        Token::KwSub => Some(OpCode::SUB),
        Token::KwMul => Some(OpCode::MUL),
        Token::KwDiv => Some(OpCode::DIV),
        Token::KwAnd => Some(OpCode::AND),
        Token::KwOr => Some(OpCode::OR),
        Token::KwXor => Some(OpCode::XOR),
        Token::KwSrl => Some(OpCode::SRL),
        Token::KwSll => Some(OpCode::SLL),
        Token::KwSrli => Some(OpCode::SRLI),
        Token::KwSlli => Some(OpCode::SLLI),
        Token::KwNot => Some(OpCode::NOT),
        Token::KwJ => Some(OpCode::J),
        Token::KwJi => Some(OpCode::JI),
        Token::KwJil => Some(OpCode::JIL),
        Token::KwJzi => Some(OpCode::JZI),
        Token::KwJnzi => Some(OpCode::JNZI),
        Token::KwJlzi => Some(OpCode::JLZI),
        Token::KwJgzi => Some(OpCode::JGZI),
        Token::KwSyscalli => Some(OpCode::SYSCALLI),
        Token::Label
            | Token::AddrToLabel
            | Token::Reg
            | Token::Hex
            | Token::Int
            | Token::Comma
            | Token::OpAdd
            | Token::OpSub
            | Token::OpMul
            | Token::OpDiv
            | Token::OpOpenBracket
            | Token::OpCloseBracket
            | Token::NewLine
            | Token::Error => None
    }
}

pub fn get_instruction_parse_type(op_code: OpCode) -> Option<InstructionParseType> {
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
            | OpCode::SLL => Some(InstructionParseType::TwoRegisters),
        OpCode::SRLI
            | OpCode::SLLI
            | OpCode::JZI
            | OpCode::JNZI
            | OpCode::JLZI
            | OpCode::JGZI
            | OpCode::LI => Some(InstructionParseType::RegisterAndImmediate),
        OpCode::NOT
            | OpCode::J => Some(InstructionParseType::Register),
        OpCode::SYSCALLI
            | OpCode::JI
            | OpCode::JIL => Some(InstructionParseType::Immediate),
    }
}

//pub fn parse(lex: &mut Lexer<Token>) {
//}

#[cfg(test)]
mod tests {
    use super::Token;
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
    fn addrToLabel() {
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
}
