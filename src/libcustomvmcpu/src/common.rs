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

use num_derive::FromPrimitive;    

/// Registers
#[derive(PartialEq, PartialOrd, Debug, Clone, Copy, FromPrimitive)]
#[repr(u8)]
pub enum Register
{
    /// Generel purpose registers
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,

    /// Stack pointer
    SP,

    /// Instruction pointer - read-only
    IP,

    /// Return instruction pointer (return-address) - read-only
    RA,

    /// Error code register - read-only
    ERR,
}

pub const LAST_REGISTER: Register = Register::ERR;

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy, FromPrimitive)]
#[repr(u8)]
pub enum OpCode {
    /// Copy from register to register
    ///
    /// # Example
    ///
    /// Copy value from register `$r0` to register `$r1`:
    ///
    /// 
    /// cpy $r0, $r1
    /// 
    CPY,
    /// Load from memory into register
    ///
    /// # Example
    ///
    /// Copy 32-bit of memory at value from register `$r1` into register `$r0`:
    ///
    /// 
    /// lw $r0, $r1
    /// 
    ///
    /// This only works when the left register is not read-only
    LW,

    /// Store register into memory
    ///
    /// # Example
    ///
    /// Copy 32-bit value of register `$r0` into memory at value of register `$r1`:
    ///
    /// 
    /// sw $r0, $r1
    /// 
    SW,

    /// Load from memory into register
    ///
    /// # Example
    ///
    /// Copy 16-bit of memory at value from register `$r1` into register `$r0`:
    ///
    /// `
    /// lh $r0 , $r1
    /// 
    LH,

    /// Store register into memory
    ///
    /// # Example
    ///
    /// Copy 16-bit value of register `$r0` into memory at value of register `$r1`:
    ///
    /// 
    /// sh $r0, $r1
    /// 
    SH,

    /// Load from memory into register
    ///
    /// # Example
    ///
    /// Load 8-bit of memory at value from register `$r1` into register `$r0`:
    ///
    /// 
    /// lb $r0, $r1
    /// 
    LB,

    /// Store register into memor
    ///
    /// # Example
    ///
    /// Copy 8-bit value of register `$r0` into memory at value of register `$r1`:
    ///
    /// 
    /// sh $r0, $r1
    /// y
    SB,

    /// Load from immediate value (value is in instruction)
    ///
    /// # Example
    ///
    /// Copy immediate value into register `$r0`:
    ///
    /// 
    /// li $r0, 2048
    /// 
    LI,

    /// Add values of two registers
    ///
    /// # Example
    /// 
    /// Add registers `$r0` and `$r1` together and store result in `$r0`:
    ///
    /// 
    /// li $r0, $r1
    /// 
    ADD,

    /// Subtract values of two registers
    ///
    /// # Example
    ///
    /// Subtract `$r1` from `$r0` and store result in `$r0`:
    ///
    /// 
    /// sub $r0, $r1
    /// 
    SUB,

    /// Multiply values of two registers
    ///
    /// # Example
    ///
    /// Multiple `$r0` and `$r1` and store result in `$r0`:
    ///
    /// 
    /// mul $r0, $r1
    /// 
    MUL,

    /// Divide values of two registers
    ///
    /// # Example
    ///
    /// Divide `$r0` through `$r1` and store result in `$r0`:
    ///
    /// 
    /// div $r0, $r1
    /// 
    DIV,

    /// Perform logical and on two registers
    ///
    /// # Example
    ///
    /// Perform logical and on `$r0` and `$r0` and store result in `$r0`:
    ///
    /// 
    /// and $r0, $r1
    /// 
    AND,

    /// Perform logical or on two registers
    ///
    /// # Example
    ///
    /// Perform logical or on `$r0` and `$r0` and store result in `$r0`:
    ///
    /// 
    /// or $r0, $r1
    /// 
    OR,

    /// Perform logical xor on two registers
    ///
    /// # Example
    ///
    /// Perform logical xor on `$r0` and `$r0` and store result in `$r0`:
    ///
    /// 
    /// xor $r0, $r1
    /// 
    XOR,

    /// Perform logical not on on register
    ///
    /// # Example
    ///
    /// Perform logical not on `$r0` and store result in `$r0`:
    ///
    /// 
    /// not $r0
    /// 
    NOT,

    /// Perform unconditional jump to memory at register value
    ///
    /// # Example
    ///
    /// Perform unconditional jump to value of result `$r0`:
    ///
    /// 
    /// j $r0
    /// 
    J,

    /// Perform unconditional jump to memory at immediate value
    ///
    /// # Example
    ///
    /// Perform unconditional jump to memory at immediate value 16
    ///
    /// 
    /// ji 16
    /// 
    JI,

    /// Perform unconditional jump to memory at immediate value and store
    /// next instruction address (current $ip) into register $ra
    JIL,
    /// Perform conditional jump to memory at immediate value
    JZI,
    /// Perform conditional jump to memory at immediate value
    JNZI,
    /// Perform conditional jump to memory at immediate value
    JLZI,
    /// Perform conditional jump to memory at immediate value
    JGZI,
    /// Perform a system call
    ///
    /// # Example
    ///
    /// Shutdown the virtual machine:
    ///
    /// 
    /// syscalli 0
    /// 
    SYSCALLI,

    /// Perform logical shift right (>>)
    ///
    /// # Example
    ///
    /// Shift value of registery `$r0` x values from register `$r1` to right
    ///
    /// 
    /// srl $r0, $r1
    /// 
    SRL,

    /// Perform logical shift left (<<)
    ///
    /// # Example
    ///
    /// Shift value of registery `$r0` x values from register `$r1` to left
    ///
    /// 
    /// sll $r0, $r1
    /// 
    SLL,

    /// Perform logical shift right (>>) with immediate
    ///
    /// # Example
    ///
    /// Shift value of registery `$r0` 4 values from register `$r1` to right
    ///
    /// 
    /// srli $r0, 4
    /// 
    SRLI,

    /// Perform logical shift left (<<) with immediate
    ///
    /// # Example
    ///
    /// Shift value of registery `$r0` 4 values from register `$r1` to left
    ///
    /// 
    /// slli $r0, 4
    /// 
    SLLI,

    /// Add values of two registers
    ///
    /// # Example
    /// 
    /// Add registers `$r0` and 10 together and store result in `$r0`:
    ///
    /// 
    /// addi $r0, 10
    /// 
    ADDI,

    /// Subtract values of two registers
    ///
    /// # Example
    ///
    /// Subtract 10 from `$r0` and store result in `$r0`:
    ///
    /// 
    /// subi $r0, 10
    /// 
    SUBI,

    /// Multiply values of two registers
    ///
    /// # Example
    ///
    /// Multiple `$r0` and 10 and store result in `$r0`:
    ///
    /// 
    /// muli $r0, 10
    /// 
    MULI,

    /// Divide values of two registers
    ///
    /// # Example
    ///
    /// Divide `$r0` through 10 and store result in `$r0`:
    ///
    /// 
    /// divi $r0, 10
    /// 
    DIVI,
}

impl ToString for OpCode {
    fn to_string(&self) -> String {
        (match self {
            Self::CPY => "cpy",
            Self::LW => "lw",
            Self::SW => "sw",
            Self::LH => "lh",
            Self::SH => "sh",
            Self::LB => "lb",
            Self::SB => "sb",
            Self::LI => "li",
            Self::ADD => "add",
            Self::SUB => "sub",
            Self::MUL => "mul",
            Self::DIV => "div",
            Self::AND => "and",
            Self::OR => "or",
            Self::XOR => "xor",
            Self::NOT => "not",
            Self::J => "j",
            Self::JI => "ji",
            Self::JIL => "jil",
            Self::JZI => "jzi",
            Self::JNZI => "jnzi",
            Self::JLZI => "jlzi",
            Self::JGZI => "jgzi",
            Self::SYSCALLI => "syscalli",
            Self::SRL => "srl",
            Self::SLL => "sll",
            Self::SRLI => "srli",
            Self::SLLI => "slli",
            Self::ADDI => "addi",
            Self::SUBI => "subi",
            Self::MULI => "muli",
            Self::DIVI => "divi",
        }).to_string()
    }
}

pub const LAST_OP_CODE: OpCode = OpCode::SYSCALLI;

/// Errors that can occur
#[derive(PartialEq, PartialOrd, Debug, Clone, Copy, FromPrimitive)]
#[repr(u32)]
pub enum Error {
    /// No error occured
    NoError,

    /// Opcode of instruction is invalid (Operation code)
    /// 
    /// # Example
    ///
    /// The instruction `0xFF000000` (OpCode is is `0x00`) is invalid.
    OpCode,

    /// Invalid register
    ///
    /// # Example
    ///
    /// The instruction `0x1000000F` uses the register `0x0F`, which doesn't
    /// exist.
    Register,

    /// Invalid syscall
    ///
    /// # Example
    ///
    /// The instruction `0x170000FF` used the syscall 255, which is invalid.
    Syscall,

    /// Memory (Out-of-bounds)
    Memory,

    /// Registers are read-only
    ReadonlyRegister,

    /// Divisor cannot be 0
    DivisorNotZero,
}

pub const ERROR_START_NUM: u32 = 32000;
