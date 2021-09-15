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
    // Generel purpose registers
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,

    // Stack pointer
    SP,

    // Instruction pointer
    IP,

    // Return instruction pointer (return-address)
    RA,

    // Error code register
    ERR,
}

pub const LAST_REGISTER: Register = Register::ERR;

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy, FromPrimitive)]
#[repr(u8)]
pub enum OpCode {
    /// Copy from register to register
    CPY,
    /// Load from memory into register
    LW,
    /// Store register into memory
    SW,
    /// Load from memory into register
    LH,
    /// Store register into memory
    SH,
    /// Load from memory into register
    LB,
    /// Store register into memory
    SB,
    /// Load from immediate value (value is in instruction)
    LI,
    /// Add values of two registers
    ADD,
    /// Subtract values of two registers
    SUB,
    /// Multiply values of two registers
    MUL,
    /// Divide values of two registers
    DIV,
    /// Perform logical and on two registers
    AND,
    /// Perform logical or on two registers
    OR,
    /// Perform logical xor on two registers
    XOR,
    /// Perform logical not on on register
    NOT,
    /// Perform unconditional jump to memory at register value
    J,
    /// Perform unconditional jump to memory at immediate value
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
    SYSCALLI,
    /// Perform logical shift right (>>)
    SRL,
    /// Perform logical shift left (<<)
    SLL,
}

pub const LAST_OP_CODE: OpCode = OpCode::SYSCALLI;

/// Errors that can occur
#[derive(PartialEq, PartialOrd, Debug, Clone, Copy, FromPrimitive)]
#[repr(u32)]
pub enum Error {
    /// No error occured
    NoError,

    /// Opcode of instruction is invalid
    OpCode,

    /// Invalid register
    Register,

    /// Invalid syscall
    Syscall,

    /// Memory (Out-of-bounds)
    Memory,

    /// Registers are read-only
    ReadonlyRegister,

    /// Divisor cannot be 0
    DivisorNotZero,
}

pub const ERROR_START_NUM: u32 = 32000;
