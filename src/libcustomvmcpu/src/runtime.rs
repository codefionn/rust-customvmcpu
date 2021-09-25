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

use std::convert::TryFrom;
use std::convert::TryInto;
use std::slice::SliceIndex;
use num_traits::FromPrimitive;
use super::common::{OpCode, Register, Error, LAST_REGISTER, ERROR_START_NUM};

/// Instruction interpreter (implementation for machine code and assembler)
pub trait Interpreter {
    /// Read from memory address
    #[must_use]
    fn read_u32(&self, pos: u32) -> Option<u32>;

    /// Write to memory address
    #[must_use]
    fn write_u32(&mut self, pos: u32, value: u32) -> bool;

    /// Read from memory address
    #[must_use]
    fn read_u16(&self, pos: u32) -> Option<u16>;

    /// Write to memory address
    #[must_use]
    fn write_u16(&mut self, pos: u32, value: u16) -> bool;

    /// Read from memory address
    #[must_use]
    fn read_u8(&self, pos: u32) -> Option<u8>;

    /// Write to memory address
    #[must_use]
    fn write_u8(&mut self, pos: u32, value: u8) -> bool;

    /// Must memory
    fn len(&self) -> u32;
}

/// 4 MiB is "RAM"
pub const BINARY_INTERPRETER_MEM_SIZE: u32 = 1024 * 1024 * 4;

#[derive(PartialEq, Debug)]
pub struct BinaryInterpreter {
    memory: Vec<u8>,
}

impl BinaryInterpreter {
    pub fn new() -> BinaryInterpreter {
        let memory = vec![0; BINARY_INTERPRETER_MEM_SIZE as usize];
        BinaryInterpreter { memory }
    }

    #[allow(unused_must_use)] // Ignoring is evil, but it's checked upfront
    pub fn new_with_program(program: &[u32]) -> Option<BinaryInterpreter> {
        let mut result = Self::new();
        if program.len() > result.memory.len() {
            eprintln!("Program length must be smaller than memory");
            return None;
        }

        let start_pos: u32 = 0;
        for pos in 0..program.len() {
            result.write_u32(pos as u32 * 4 + start_pos, program[pos]);
        }

        return Some(result);
    }

    /// Initializes BinaryInterpreter with initial memory
    pub fn new_with_initial(initial_memory: &Vec<u8>) -> Option<BinaryInterpreter> {
        let mut result = Self::new();
        let slice_from_memory = result.memory.get_mut(0..initial_memory.len());
        return if let Some(slice_from_memory) = slice_from_memory {
            slice_from_memory.copy_from_slice(&initial_memory);
            Some(result)
        }
        else {
            None
        };
    }
}

impl Interpreter for BinaryInterpreter {
    #[must_use]
    fn read_u32(&self, pos: u32) -> Option<u32> {
        let result = self.memory.get(pos as usize..(pos as usize + 4));
        return if let Some(result) = result {
            Some(u32::from_le_bytes(result.try_into().expect("Unexpected error")))
        }
        else {
            None
        }
    }

    #[must_use]
    fn write_u32(&mut self, pos: u32, value: u32) -> bool {
        let result = self.memory.get_mut(pos as usize..pos as usize + 4);
        return if let Some(result) = result {
            result.copy_from_slice(&u32::to_le_bytes(value));
            true
        }
        else {
            false
        }
    }

    #[must_use]
    fn read_u16(&self, pos: u32) -> Option<u16> {
        let result = self.memory.get(pos as usize..(pos as usize + 2));
        return if let Some(result) = result {
            Some(u16::from_le_bytes(result.try_into().expect("Unexpected error")))
        }
        else {
            None
        }
    }

    #[must_use]
    fn write_u16(&mut self, pos: u32, value: u16) -> bool {
        let result = self.memory.get_mut(pos as usize..pos as usize + 2);
        return if let Some(result) = result {
            result.copy_from_slice(&u16::to_le_bytes(value));
            true
        }
        else {
            false
        }
    }

    #[must_use]
    fn read_u8(&self, pos: u32) -> Option<u8> {
        let result = self.memory.get(pos as usize);
        return if let Some(&result) = result {
            Some(result)
        }
        else {
            None
        }
    }

    #[must_use]
    fn write_u8(&mut self, pos: u32, value: u8) -> bool {
        let result = self.memory.get_mut(pos as usize);
        return if let Some(result) = result {
            *result = value;
            true
        }
        else {
            false
        }
    }

    fn len(&self) -> u32 {
        u32::try_from(self.memory.len()).expect("Less than u32::MAX expected")
    }
}

/// Virtual machine to execute machine code on
pub struct VirtualMachine<InterpreterImpl: Interpreter>
{
    interpreter: InterpreterImpl,
    registers: [u32; LAST_REGISTER as usize + 1],
    running: bool,
}

impl<InterpreterImpl: Interpreter> VirtualMachine<InterpreterImpl> {
    /// Create a new virtual machine
    pub fn new(interpreter: InterpreterImpl) -> VirtualMachine<InterpreterImpl> {
        let mut result = VirtualMachine { interpreter, registers: [0; LAST_REGISTER as usize + 1], running: false };
        result.write_register_value(Register::SP, result.interpreter.len());
        result
    }

    /// Reset all registers (for restarting the machine)
    pub fn reset(&mut self) {
        for reg in self.registers.iter_mut() {
            *reg = 0;
        }
    }

    /// Execute program with entry point at 0
    /// If result is greater than ERROR_START_NUM than it's a CPU error
    pub fn execute_first(&mut self) -> u32 {
        self.execute(0)
    }

    /// Execute program with entry point at pos
    /// If result is greater than ERROR_START_NUM than it's a CPU error
    pub fn execute(&mut self, pos: u32) -> u32 {
        self.running = true;
        self.write_register_value(Register::IP, pos);
        self.write_register_value(Register::ERR, Error::NoError as u32);

        loop {
            let instruction = self.interpreter.read_u32(self.read_register_value(Register::IP));
            if let Some(parsed_instruction) = instruction {
                self.interpret_instruction(parsed_instruction);
            }
            else {
                self.write_register_value(Register::ERR, Error::Memory as u32);
                break;
            }

            if self.read_register_value(Register::ERR) != Error::NoError as u32 || !self.running {
                break;
            }

            self.write_register_value(Register::IP, self.read_register_value(Register::IP).wrapping_add(4));
        } 

        let error_value = self.read_register_value(Register::ERR);
        return if error_value == (Error::NoError as u32) {
            self.read_register_value(Register::R1)
        }
        else {
            error_value + ERROR_START_NUM
        }
    }

    fn interpret_instruction(&mut self, instruction: u32) {
        let opcode = Self::get_opcode(instruction);
        let opcode = OpCode::from_u8(opcode);
        if let Some(opcode) = opcode {
            //println!("Executing opcode: {:?}", opcode);

            match opcode {
                OpCode::SYSCALLI => {
                    self.write_next_instruction_address();
                    self.syscall(Self::get_immediate(instruction))
                },
                OpCode::CPY => {
                    self.binary_register_operation(instruction, |this: &mut Self, reg0, reg1|
                        this.write_user_register_value(reg0, this.read_user_register_value(reg1))
                    );
                },
                // Load-store
                OpCode::LW => {
                    self.binary_register_operation(instruction, |this: &mut Self, reg0, reg1|
                        if let Some(result) = this.interpreter.read_u32(this.read_user_register_value(reg1)) {
                            this.write_user_register_value(reg0, result);
                        }
                        else {
                            this.write_error(Error::Memory);
                        }
                    );
                },
                OpCode::SW => {
                    self.binary_register_operation(instruction, |this: &mut Self, reg0, reg1|
                        if !this.interpreter.write_u32(this.read_user_register_value(reg1), this.read_user_register_value(reg0)) {
                            this.write_error(Error::Memory);
                        }
                    );
                },
                OpCode::LH => {
                    self.binary_register_operation(instruction, |this: &mut Self, reg0, reg1|
                        if let Some(result) = this.interpreter.read_u16(this.read_user_register_value(reg1)) {
                            this.write_user_register_value(reg0, result as u32);
                        }
                        else {
                            this.write_error(Error::Memory);
                        }
                    );
                },
                OpCode::SH => {
                    self.binary_register_operation(instruction, |this: &mut Self, reg0, reg1|
                        if !this.interpreter.write_u16(this.read_user_register_value(reg1), (this.read_user_register_value(reg0) & 0x0000FFFF).try_into().expect("Unexpected error")) {
                            this.write_error(Error::Memory);
                        }
                    );
                },
                OpCode::LB => {
                    self.binary_register_operation(instruction, |this: &mut Self, reg0, reg1|
                        if let Some(result) = this.interpreter.read_u8(this.read_user_register_value(reg1)) {
                            this.write_user_register_value(reg0, result as u32);
                        }
                        else {
                            this.write_error(Error::Memory);
                        }
                    );
                },
                OpCode::SB => {
                    self.binary_register_operation(instruction, |this: &mut Self, reg0, reg1|
                        if !this.interpreter.write_u8(this.read_user_register_value(reg1), (this.read_user_register_value(reg0) & 0x000000FF).try_into().expect("Unexpected error")) {
                            this.write_error(Error::Memory);
                        }
                    );
                },
                OpCode::LI => {
                    let (reg0, imm1) = Self::get_register_and_twos_complement_immediate(instruction);
                    if let Some(reg_value0) = Register::from_u8(reg0) {
                        self.write_user_register_value(reg_value0, imm1);
                    }
                    else {
                        eprintln!("Register {:?} does not exists!", reg0);
                        self.write_error(Error::Register);
                    }
                },
                OpCode::LWI => {
                    self.binary_register_and_immediate_operation(instruction, |this: &mut Self, reg, imm|
                        if let Some(result) = this.interpreter.read_u32(imm) {
                            this.write_user_register_value(reg, result);
                        }
                        else {
                            this.write_error(Error::Memory);
                        }
                    );
                },
                OpCode::SWI => {
                    self.binary_register_and_immediate_operation(instruction, |this: &mut Self, reg, imm|
                        if !this.interpreter.write_u32(imm, this.read_user_register_value(reg)) {
                            this.write_error(Error::Memory);
                        }
                    );
                },
                OpCode::LHI => {
                    self.binary_register_and_immediate_operation(instruction, |this: &mut Self, reg, imm|
                        if let Some(result) = this.interpreter.read_u16(imm) {
                            this.write_user_register_value(reg, result as u32);
                        }
                        else {
                            this.write_error(Error::Memory);
                        }
                    );
                },
                OpCode::SHI => {
                    self.binary_register_and_immediate_operation(instruction, |this: &mut Self, reg, imm|
                        if !this.interpreter.write_u16(imm, (this.read_user_register_value(reg) & 0x0000FFFF).try_into().expect("Unexpected error")) {
                            this.write_error(Error::Memory);
                        }
                    );
                },
                OpCode::LBI => {
                    self.binary_register_and_immediate_operation(instruction, |this: &mut Self, reg, imm|
                        if let Some(result) = this.interpreter.read_u8(imm) {
                            this.write_user_register_value(reg, result as u32);
                        }
                        else {
                            this.write_error(Error::Memory);
                        }
                    );
                },
                OpCode::SBI => {
                    self.binary_register_and_immediate_operation(instruction, |this: &mut Self, reg, imm|
                        if !this.interpreter.write_u8(imm, (this.read_user_register_value(reg) & 0x000000FF).try_into().expect("Unexpected error")) {
                            this.write_error(Error::Memory);
                        }
                    );
                },
                // Arithmetics
                OpCode::ADD => {
                    self.binary_register_operation_write0(instruction, |_: &mut Self, x, y| x.wrapping_add(y));
                },
                OpCode::SUB => {
                    self.binary_register_operation_write0(instruction, |_: &mut Self, x, y| x.wrapping_sub(y));
                },
                OpCode::MUL => {
                    self.binary_register_operation_write0(instruction, |_: &mut Self, x, y| x.wrapping_mul(y));
                },
                OpCode::DIV => {
                    self.binary_register_operation_write0(instruction,
                        |this: &mut Self, x, y|
                            if y == 0 {
                                this.write_error(Error::DivisorNotZero);
                                0
                            } else {
                                x / y
                            }
                    );
                },
                OpCode::ADDI => {
                    self.binary_register_and_immediate_operation_write0(instruction, |_: &mut Self, x, y| x.wrapping_add(y));
                },
                OpCode::SUBI => {
                    self.binary_register_and_immediate_operation_write0(instruction, |_: &mut Self, x, y| x.wrapping_sub(y));
                },
                OpCode::MULI => {
                    self.binary_register_and_immediate_operation_write0(instruction, |_: &mut Self, x, y| x.wrapping_mul(y));
                },
                OpCode::DIVI => {
                    self.binary_register_and_immediate_operation_write0(instruction,
                        |this: &mut Self, x, y|
                            if y == 0 {
                                this.write_error(Error::DivisorNotZero);
                                0
                            } else {
                                x / y
                            }
                    );
                },
                // Unconditional jumps
                OpCode::J => {
                    let reg = Self::get_registers(instruction);
                    if let Some(reg_value) = Register::from_u8(reg) {
                        let address = self.read_user_register_value(reg_value);
                        self.write_register_value(Register::IP, address.wrapping_sub(4)); // Minus 4 because this will be added after every cycle
                    }
                    else {
                        eprintln!("Register {:?} does not exists!", reg);
                        self.write_error(Error::Register);
                    }
                },
                OpCode::JI => {
                    let address = Self::get_immediate(instruction);
                    self.write_register_value(Register::IP, address.wrapping_sub(4)); // Minus 4 because this will be added after every cycle
                }
                OpCode::JIL => {
                    let address = Self::get_immediate(instruction);
                    self.write_register_value(Register::RA, self.read_register_value(Register::IP).wrapping_add(4)); // Plus 4 because it points to the next instruction
                    self.write_register_value(Register::IP, address.wrapping_sub(4)); // Minus 4 because this will be added after every cycle
                },
                OpCode::JZI => {
                    self.unary_check_write_ip(instruction, |this: &mut Self, x| x == 0);
                },
                OpCode::JNZI => {
                    self.unary_check_write_ip(instruction, |this: &mut Self, x| x != 0);
                },
                OpCode::JLZI => {
                    self.unary_check_write_ip(instruction,
                        |this: &mut Self, x| i32::from_le_bytes(u32::to_le_bytes(x)) < 0
                    );
                },
                OpCode::JGZI => {
                    self.unary_check_write_ip(instruction,
                        |this: &mut Self, x| i32::from_le_bytes(u32::to_le_bytes(x)) > 0
                    );
                },
                OpCode::AND => {
                    self.binary_register_operation_write0(instruction, |_: &mut Self, x, y| x & y);
                },
                OpCode::OR => {
                    self.binary_register_operation_write0(instruction, |_: &mut Self, x, y| x | y);
                },
                OpCode::XOR => {
                    self.binary_register_operation_write0(instruction, |_: &mut Self, x, y| x ^ y);
                },
                OpCode::NOT => {
                    let reg = Self::get_registers(instruction);
                    if let Some(reg_value) = Register::from_u8(reg) {
                        let val = self.read_user_register_value(reg_value);
                        self.write_user_register_value(reg_value, !val);
                    }
                    else {
                        eprintln!("Register {:?} does not exists!", reg);
                        self.write_error(Error::Register);
                    }
                },
                OpCode::SRL => {
                    self.binary_register_operation_write0(instruction, |_: &mut Self, x, y| x >> y);
                },
                OpCode::SLL => {
                    self.binary_register_operation_write0(instruction, |_: &mut Self, x, y| x << y);
                },
                OpCode::SRLI => {
                    self.binary_register_and_immediate_operation_write0(instruction,
                        |_: &mut Self, x, y| x >> y);
                },
                OpCode::SLLI => {
                    self.binary_register_and_immediate_operation_write0(instruction,
                        |_: &mut Self, x, y| x << y);
                }
            }
        }
        else {
            eprintln!("Instruction {:?} does not exist!", opcode);
            self.write_error(Error::OpCode);
            return;
        }
    }

    /// Writes the address - 4 to register $ip, if `unary_op` evaluates to
    /// true.
    fn unary_check_write_ip(&mut self, instruction: u32, unary_op: fn (&mut Self, u32) -> bool) {
      let (reg, imm) = Self::get_register_and_immediate(instruction);
      if let Some(reg_value) = Register::from_u8(reg) {
          let val = self.read_user_register_value(reg_value);
          if unary_op(self, val) {
            self.write_register_value(Register::IP, imm.wrapping_sub(4));
          }
      }
      else {
          eprintln!("Register {:?} does not exists!", reg);
          self.write_error(Error::Register);
      }
    }

    fn binary_register_and_immediate_operation_write0(&mut self, instruction: u32, binary_op: fn (&mut Self, u32, u32) -> u32) {
      let (reg, imm) = Self::get_register_and_immediate(instruction);
      if let Some(reg_value) = Register::from_u8(reg) {
          let val = self.read_user_register_value(reg_value);
          let result = binary_op(self, val, imm);
          self.write_user_register_value(reg_value, result);
      }
      else {
          eprintln!("Register {:?} does not exists!", reg);
          self.write_error(Error::Register);
      }
    }

    fn binary_register_and_immediate_operation(&mut self, instruction: u32, binary_op: fn (&mut Self, Register, u32)) {
      let (reg, imm) = Self::get_register_and_immediate(instruction);
      if let Some(reg_value) = Register::from_u8(reg) {
          binary_op(self, reg_value, imm);
      }
      else {
          eprintln!("Register {:?} does not exists!", reg);
          self.write_error(Error::Register);
      }
    }

    /// Combines both values of the two registers parsed from the instruction with the function
    /// `binary_op` and writes the result in the first registers
    fn binary_register_operation_write0(&mut self, instruction: u32, binary_op: fn (&mut Self, u32, u32) -> u32) {
      let (reg0, reg1) = Self::get_two_registers(instruction);
      if let (Some(reg_value0), Some(reg_value1)) = (Register::from_u8(reg0), Register::from_u8(reg1)) {
          let val0 = self.read_user_register_value(reg_value0);
          let val1 = self.read_user_register_value(reg_value1);
          let result = binary_op(self, val0, val1);
          self.write_user_register_value(reg_value0, result);
      }
      else {
          eprintln!("Register {:?} or {:?} does not exists!", reg0, reg1);
          self.write_error(Error::Register);
      }
    }

    fn binary_register_operation(&mut self, instruction: u32, binary_op: fn (&mut Self, Register, Register)) {
      let (reg0, reg1) = Self::get_two_registers(instruction);
      if let (Some(reg_value0), Some(reg_value1)) = (Register::from_u8(reg0), Register::from_u8(reg1)) {
          binary_op(self, reg_value0, reg_value1);
      }
      else {
          eprintln!("Register {:?} or {:?} does not exists!", reg0, reg1);
          self.write_error(Error::Register);
      }
    }

    /// Saves the address of the next instruction in $ra
    #[inline(always)]
    fn write_next_instruction_address(&mut self) {
        self.write_register_value(Register::RA, self.read_register_value(Register::IP) + 4);
    }

    /// Check if register is read-only
    fn is_readonly(reg: Register) -> bool {
        return match reg {
            Register::IP | Register::ERR => true,
            _ => false
        }
    }

    #[inline(always)]
    fn write_error(&mut self, err: Error) {
        self.write_register_value(Register::ERR, err as u32);
    }

    /// Write to unkonwn register value
    #[inline(always)]
    fn write_unknown_register_value(&mut self, reg: u8, value: u32) {
        if let Some(reg_value) = Register::from_u8(reg) {
            self.write_register_value(reg_value, value);
        }
        else {
            eprintln!("Unkown register {:?}", reg);
            self.write_register_value(Register::ERR, Error::Register as u32);
        }
    }

    // If the user (program) writes to register reg
    #[inline(always)]
    pub fn write_user_register_value(&mut self, reg: Register, value: u32) {
        if Self::is_readonly(reg) {
           eprintln!("Register {:?} is read-only", reg);
           self.write_error(Error::ReadonlyRegister);
        }
        else {
           self.write_register_value(reg, value);
        }
    }

    /// Writes value value to register reg
    #[inline(always)]
    pub fn write_register_value(&mut self, reg: Register, value: u32) {
        self.registers[reg as usize] = value;
    }

    #[inline(always)]
    fn read_user_register_value(&self, reg: Register) -> u32 {
        self.read_register_value(reg)
    }

    /// Reads value from register reg
    #[inline(always)]
    pub fn read_register_value(&self, reg: Register) -> u32 {
        self.registers[reg as usize]
    }

    fn syscall(&mut self, syscall: u32) {
        match syscall {
            0 => {
                self.running = false;
            },
            _ => {
                eprintln!("Unknown syscall {:?}", syscall);
                self.write_register_value(Register::ERR, Error::Syscall as u32);
            }
        }
    }

    #[inline(always)]
    fn get_opcode(instruction: u32) -> u8 {
        u8::try_from((instruction & 0xFF000000) >> (3 * 8)).expect("Unexpected failure!")
    }


    #[inline(always)]
    fn get_immediate(instruction: u32) -> u32 {
        instruction & 0x00FFFFFF
    }

    #[inline(always)]
    fn get_registers(instruction: u32) -> u8 {
        u8::try_from(instruction & 0x0000000F).expect("Unexpected failure!")
    }

    #[inline(always)]
    fn get_two_registers(instruction: u32) -> (u8, u8) {
        (
            u8::try_from((instruction & 0x00F00000) >> (2 * 8 + 4)).expect("Unexpected failure!"),
            u8::try_from(instruction & 0x0000000F).expect("Unexpected failure!"),
        )
    }

    #[inline(always)]
    fn get_register_and_immediate(instruction: u32) -> (u8, u32) {
        (
            u8::try_from((instruction & 0x00F00000) >> (2 * 8 + 4)).expect("Unexpected failure!"),
            instruction & 0x000FFFFF
        )
    }

    #[inline(always)]
    fn get_register_and_twos_complement_immediate(instruction: u32) -> (u8, u32) {
        (
            u8::try_from((instruction & 0x00F00000) >> (2 * 8 + 4)).expect("Unexpected failure!"),
            Self::get_u32_from_immediate(instruction & 0x000FFFFF, 0x000FFFFF, 0x00080000)
        )
    }

    #[inline(always)]
    fn get_two_register_and_immediate(instruction: u32) -> (u8, u8, u32) {
        (
            u8::try_from((instruction & 0x00F00000) >> (2 * 8 + 4)).expect("Unexpected failure!"),
            u8::try_from((instruction & 0x000F0000) >> (2 * 8 + 0)).expect("Unexpected failure!"),
            instruction & 0x0000FFFF
        )
    }

    /// Returns u32 from immediate. Immediate is a twos complement!
    #[inline(always)]
    fn get_u32_from_immediate(imm: u32, bitmask: u32, check_negative_bitmask: u32) -> u32 {
        if imm & check_negative_bitmask == 0 { // Positive
            imm
        }
        else {
            imm | !bitmask // Two's complement -> Add 1 to the start
        }
    }

    pub fn get_interpreter(&mut self) -> &InterpreterImpl {
        &self.interpreter
    }

    pub fn get_interpreter_mut(&mut self) -> &mut InterpreterImpl {
        &mut self.interpreter
    }
}

pub mod utils {
    use super::{OpCode, Register};

    pub const fn create_instruction_register(opcode: OpCode, reg: Register) -> u32
    {
        ((opcode as u32) << 3 * 8) | (reg as u32)
    }

    pub const fn create_instruction_immediate(opcode: OpCode, imm: u32) -> u32
    {
        ((opcode as u32) << 3 * 8) | imm
    }

    pub const fn create_instruction_register_and_immediate(opcode: OpCode, reg: Register, imm: u32) -> u32 {
        ((opcode as u32)  << 3 * 8) | ((reg as u32) << 2 * 8 + 4) | (imm & 0x000FFFFF)
    }
    
    pub const fn create_instruction_two_registers(opcode: OpCode, reg0: Register, reg1: Register) -> u32 {
        ((opcode as u32)  << 3 * 8) | ((reg0 as u32) << 2 * 8 + 4) | (reg1 as u32)
    }
    
    pub const fn create_instruction_two_registers_and_immediate(opcode: OpCode, reg0: Register, reg1: Register, imm: u32) -> u32 {
        ((opcode as u32)  << 3 * 8) | ((reg0 as u32) << 2 * 8 + 4) | ((reg1 as u32) << 2 * 8) | (imm & 0x0000FFFF)
    }
}

pub type BinaryVirtualMachine = VirtualMachine<BinaryInterpreter>;

#[cfg(test)]
mod tests {
    use super::{OpCode, BinaryInterpreter, BinaryVirtualMachine, Interpreter, Register, utils, Error, ERROR_START_NUM, BINARY_INTERPRETER_MEM_SIZE};

    const SYSCALLI_EXIT_INSTRUCTION: u32 = u32::to_le((OpCode::SYSCALLI as u32) << 3 * 8);
    const LOAD_0_IN_R1_INSTRUCTION: u32 = utils::create_instruction_register_and_immediate(OpCode::LI, Register::R1, 0);

    #[test]
    fn get_opcode() {
        assert_eq!(OpCode::SYSCALLI as u8, BinaryVirtualMachine::get_opcode(SYSCALLI_EXIT_INSTRUCTION));
    }

    #[test]
    fn syscall_exit() {
        let syscode_inst = SYSCALLI_EXIT_INSTRUCTION;
        let interpreter = BinaryInterpreter::new_with_program(&[syscode_inst]).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.read_register_value(Register::IP));
        assert_eq!(Some(syscode_inst), vm.get_interpreter().read_u32(0));
        assert_eq!(0, vm.execute_first());
        assert_eq!(0, vm.read_register_value(Register::IP));

        let syscode_inst = SYSCALLI_EXIT_INSTRUCTION;
        let interpreter = BinaryInterpreter::new_with_program(&[
            syscode_inst,
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 32)
        ]).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.read_register_value(Register::IP));
        assert_eq!(Some(syscode_inst), vm.get_interpreter().read_u32(0));
        assert_eq!(0, vm.execute_first());
        assert_eq!(0, vm.read_register_value(Register::R0));
        assert_eq!(0, vm.read_register_value(Register::IP));

    }

    #[test]
    fn li_r0() {
        let inst = utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 564);
        let interpreter = BinaryInterpreter::new_with_program(&[inst, LOAD_0_IN_R1_INSTRUCTION, SYSCALLI_EXIT_INSTRUCTION]).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(564, vm.read_register_value(Register::R0));
    }

    #[test]
    fn li_r1() {
        let inst = utils::create_instruction_register_and_immediate(OpCode::LI, Register::R1, 563);
        let interpreter = BinaryInterpreter::new_with_program(&[inst, SYSCALLI_EXIT_INSTRUCTION]).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(563, vm.execute_first());
        assert_eq!(563, vm.read_register_value(Register::R1));
    }

    #[test]
    fn li_r7() {
        let inst = utils::create_instruction_register_and_immediate(OpCode::LI, Register::R7, 513);
        let interpreter = BinaryInterpreter::new_with_program(&[inst, LOAD_0_IN_R1_INSTRUCTION, SYSCALLI_EXIT_INSTRUCTION]).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(513, vm.read_register_value(Register::R7));
    }

    #[test]
    fn li_ip() {
        let inst = utils::create_instruction_register_and_immediate(OpCode::LI, Register::IP, 12);
        let interpreter = BinaryInterpreter::new_with_program(&[inst, LOAD_0_IN_R1_INSTRUCTION, SYSCALLI_EXIT_INSTRUCTION]).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(ERROR_START_NUM + (Error::ReadonlyRegister as u32), vm.execute_first());
    }

    #[test]
    fn li_err() {
        let inst = utils::create_instruction_register_and_immediate(OpCode::LI, Register::ERR, 12);
        let interpreter = BinaryInterpreter::new_with_program(&[inst, LOAD_0_IN_R1_INSTRUCTION, SYSCALLI_EXIT_INSTRUCTION]).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(ERROR_START_NUM + (Error::ReadonlyRegister as u32), vm.execute_first());
    }

    #[test]
    fn add() {
        let program: [u32; 5] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 16),
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R1, 5),
            utils::create_instruction_two_registers(OpCode::ADD, Register::R0, Register::R1),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(21, vm.read_register_value(Register::R0));
    }

    #[test]
    fn cpy() {
        let program: [u32; 4] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 16),
            utils::create_instruction_two_registers(OpCode::CPY, Register::R2, Register::R0),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(16, vm.read_register_value(Register::R2));
    }

    #[test]
    fn sub() {
        let program: [u32; 5] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 16),
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R1, 5),
            utils::create_instruction_two_registers(OpCode::SUB, Register::R0, Register::R1),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(11, vm.read_register_value(Register::R0));
    }

    #[test]
    fn mul() {
        let program: [u32; 5] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 4),
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R1, 5),
            utils::create_instruction_two_registers(OpCode::MUL, Register::R0, Register::R1),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(20, vm.read_register_value(Register::R0));
    }

    #[test]
    fn div() {
        let program: [u32; 5] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 20),
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R1, 5),
            utils::create_instruction_two_registers(OpCode::DIV, Register::R0, Register::R1),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(4, vm.read_register_value(Register::R0));

        let program: [u32; 5] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 24),
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R1, 5),
            utils::create_instruction_two_registers(OpCode::DIV, Register::R0, Register::R1),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(4, vm.read_register_value(Register::R0));
    }

    #[test]
    fn div_divisor_zero() {
      let program: [u32; 5] = [
          utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 20),
          utils::create_instruction_register_and_immediate(OpCode::LI, Register::R1, 0),
          utils::create_instruction_two_registers(OpCode::DIV, Register::R0, Register::R1),
          LOAD_0_IN_R1_INSTRUCTION,
          SYSCALLI_EXIT_INSTRUCTION
      ];

      let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
      let mut vm = BinaryVirtualMachine::new(interpreter);

      assert_eq!(ERROR_START_NUM + Error::DivisorNotZero as u32, vm.execute_first());
      assert_eq!(0, vm.read_register_value(Register::R0));
    }

    #[test]
    fn addi() {
        let program: [u32; 4] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 16),
            utils::create_instruction_register_and_immediate(OpCode::ADDI, Register::R0, 5),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(21, vm.read_register_value(Register::R0));
    }

    #[test]
    fn subi() {
        let program: [u32; 4] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 16),
            utils::create_instruction_register_and_immediate(OpCode::SUBI, Register::R0, 5),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(11, vm.read_register_value(Register::R0));
    }

    #[test]
    fn muli() {
        let program: [u32; 4] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 4),
            utils::create_instruction_register_and_immediate(OpCode::MULI, Register::R0, 5),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(20, vm.read_register_value(Register::R0));
    }

    #[test]
    fn divi() {
        let program: [u32; 4] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 20),
            utils::create_instruction_register_and_immediate(OpCode::DIVI, Register::R0, 5),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(4, vm.read_register_value(Register::R0));

        let program: [u32; 4] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 24),
            utils::create_instruction_register_and_immediate(OpCode::DIVI, Register::R0, 5),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(4, vm.read_register_value(Register::R0));
    }

    #[test]
    fn divi_divisor_zero() {
      let program: [u32; 4] = [
          utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 20),
          utils::create_instruction_register_and_immediate(OpCode::DIVI, Register::R0, 0),
          LOAD_0_IN_R1_INSTRUCTION,
          SYSCALLI_EXIT_INSTRUCTION
      ];

      let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
      let mut vm = BinaryVirtualMachine::new(interpreter);

      assert_eq!(ERROR_START_NUM + Error::DivisorNotZero as u32, vm.execute_first());
      assert_eq!(0, vm.read_register_value(Register::R0));
    }

    #[test]
    fn lw() {
        let program: [u32; 5] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R2, 4 * 4),
            utils::create_instruction_two_registers(OpCode::LW, Register::R0, Register::R2),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            0xFF00FF00
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(0xFF00FF00, vm.read_register_value(Register::R0));
    }

    #[test]
    fn sw() {
        let program: [u32; 5] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R2, 5 * 4),
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 1033),
            utils::create_instruction_two_registers(OpCode::SW, Register::R0, Register::R2),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(1033, vm.get_interpreter().read_u32(5 * 4).expect("Cannot read memory address"));
    }

    #[test]
    fn lh() {
        let program: [u32; 5] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R2, 4 * 4),
            utils::create_instruction_two_registers(OpCode::LH, Register::R0, Register::R2),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            1032 // Will be stored in [0] and [1] of integer 0124, because little endian
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(1032, vm.read_register_value(Register::R0));
    }

    #[test]
    fn sh() {
        let program: [u32; 5] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R2, 5 * 4),
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 1033),
            utils::create_instruction_two_registers(OpCode::SH, Register::R0, Register::R2),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(1033, vm.get_interpreter().read_u16(5 * 4).expect("Cannot read memory address"));
    }

    #[test]
    fn lb() {
        let program: [u32; 5] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R2, 4 * 4),
            utils::create_instruction_two_registers(OpCode::LB, Register::R0, Register::R2),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            234 // Will be stored in [0] and [1] of integer 0124, because little endian
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(234, vm.read_register_value(Register::R0));
    }

    #[test]
    fn sb() {
        let program: [u32; 5] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R2, 5 * 4),
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 234),
            utils::create_instruction_two_registers(OpCode::SB, Register::R0, Register::R2),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(234, vm.get_interpreter().read_u8(5 * 4).expect("Cannot read memory address"));
    }

    #[test]
    fn lb_partial() {
        let program: [u32; 5] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R2, 4 * 4),
            utils::create_instruction_two_registers(OpCode::LB, Register::R0, Register::R2),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            1024 + 234 // Will be stored in [0] and [1] of integer 0124, because little endian
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(234, vm.read_register_value(Register::R0));
    }

    #[test]
    fn sb_partial() {
        let program: [u32; 5] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R2, 5 * 4),
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 1024 + 234),
            utils::create_instruction_two_registers(OpCode::SB, Register::R0, Register::R2),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(234, vm.get_interpreter().read_u8(5 * 4).expect("Cannot read memory address"));
        assert_eq!(234, vm.get_interpreter().read_u32(5 * 4).expect("Cannot read memory address"));
    }

    #[test]
    fn lwi() {
        let program: [u32; 4] = [
            utils::create_instruction_register_and_immediate(OpCode::LWI, Register::R0, 3 * 4),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            0xFF00FF00
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(0xFF00FF00, vm.read_register_value(Register::R0));
    }

    #[test]
    fn swi() {
        let program: [u32; 4] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 1033),
            utils::create_instruction_register_and_immediate(OpCode::SWI, Register::R0, 4 * 4),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(1033, vm.get_interpreter().read_u32(4 * 4).expect("Cannot read memory address"));
    }

    #[test]
    fn lhi() {
        let program: [u32; 4] = [
            utils::create_instruction_register_and_immediate(OpCode::LHI, Register::R0, 3 * 4),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            1032 // Will be stored in [0] and [1] of integer 0124, because little endian
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(1032, vm.read_register_value(Register::R0));
    }

    #[test]
    fn shi() {
        let program: [u32; 4] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 1033),
            utils::create_instruction_register_and_immediate(OpCode::SHI, Register::R0, 4 * 4),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(1033, vm.get_interpreter().read_u16(4 * 4).expect("Cannot read memory address"));
    }

    #[test]
    fn lbi() {
        let program: [u32; 4] = [
            utils::create_instruction_register_and_immediate(OpCode::LBI, Register::R0, 3 * 4),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            234 // Will be stored in [0] and [1] of integer 0124, because little endian
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(234, vm.read_register_value(Register::R0));
    }

    #[test]
    fn sbi() {
        let program: [u32; 4] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 234),
            utils::create_instruction_register_and_immediate(OpCode::SBI, Register::R0, 4 * 4),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(234, vm.get_interpreter().read_u8(4 * 4).expect("Cannot read memory address"));
    }

    #[test]
    fn lbi_partial() {
        let program: [u32; 4] = [
            utils::create_instruction_register_and_immediate(OpCode::LBI, Register::R0, 3 * 4),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            1024 + 234 // Will be stored in [0] and [1] of integer 0124, because little endian
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(234, vm.read_register_value(Register::R0));
    }

    #[test]
    fn sbi_partial() {
        let program: [u32; 4] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 1024 + 234),
            utils::create_instruction_register_and_immediate(OpCode::SBI, Register::R0, 4 * 4),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(234, vm.get_interpreter().read_u8(4 * 4).expect("Cannot read memory address"));
        assert_eq!(234, vm.get_interpreter().read_u32(4 * 4).expect("Cannot read memory address"));
    }

    #[test]
    fn j() {
        let program: [u32; 7] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R2, 4 * 4),
            utils::create_instruction_register(OpCode::J, Register::R2),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 32),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(32, vm.read_register_value(Register::R0));
    }

    #[test]
    fn ji() {
        let program: [u32; 6] = [
            utils::create_instruction_immediate(OpCode::JI, 3 * 4),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 32),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(32, vm.read_register_value(Register::R0));
    }

    #[test]
    fn jil() {
        let program: [u32; 8] = [
            utils::create_instruction_immediate(OpCode::JI, 1 * 4), // nop
            utils::create_instruction_immediate(OpCode::JIL, 4 * 4),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 32),
            utils::create_instruction_two_registers(OpCode::CPY, Register::R3, Register::RA),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(32, vm.read_register_value(Register::R0));
        assert_eq!(8, vm.read_register_value(Register::R3));
    }

    #[test]
    fn jzi() {
        let program: [u32; 6] = [
            utils::create_instruction_register_and_immediate(OpCode::JZI, Register::R0, 3 * 4),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 32),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(32, vm.read_register_value(Register::R0));
    }

    #[test]
    fn jnzi() {
        let program: [u32; 7] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, u32::from_le_bytes(i32::to_le_bytes(-1))),
            utils::create_instruction_register_and_immediate(OpCode::JNZI, Register::R0, 4 * 4),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 32),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(32, vm.read_register_value(Register::R0));
    }

    #[test]
    fn li_minus_1()
    {
        let program: [u32; 5] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 4 * 4),
            utils::create_instruction_two_registers(OpCode::LW, Register::R0, Register::R0),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            u32::from_le_bytes(i32::to_le_bytes(-1))
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(-1, i32::from_le_bytes(u32::to_le_bytes(vm.read_register_value(Register::R0))));
    }

    #[test]
    fn li_minus_1_new_way()
    {
        let program: [u32; 3] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, u32::from_le_bytes(i32::to_le_bytes(-1))),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(-1, i32::from_le_bytes(u32::to_le_bytes(vm.read_register_value(Register::R0))));
    }

    #[test]
    fn jlzi() {
        let program: [u32; 9] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 8 * 4),
            utils::create_instruction_two_registers(OpCode::LW, Register::R0, Register::R0),
            utils::create_instruction_register_and_immediate(OpCode::JLZI, Register::R0, 5 * 4),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 32),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            u32::from_le_bytes(i32::to_le_bytes(-1))
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(32, vm.read_register_value(Register::R0) as i32);
    }

    #[test]
    fn jlzi_new_way() {
        let program: [u32; 7] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, u32::from_le_bytes(i32::to_le_bytes(-1))),
            utils::create_instruction_register_and_immediate(OpCode::JLZI, Register::R0, 4 * 4),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 32),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(32, vm.read_register_value(Register::R0) as i32);
    }

    #[test]
    fn jgzi() {
        let program: [u32; 7] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 1),
            utils::create_instruction_register_and_immediate(OpCode::JGZI, Register::R0, 4 * 4),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 32),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(32, vm.read_register_value(Register::R0));
    }

    #[test]
    fn and() {
        let program: [u32; 9] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 7 * 4),
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R2, 8 * 4),
            utils::create_instruction_two_registers(OpCode::LW, Register::R0, Register::R0),
            utils::create_instruction_two_registers(OpCode::LW, Register::R2, Register::R2),
            utils::create_instruction_two_registers(OpCode::AND, Register::R0, Register::R2),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            0x0000FFFF,
            0xFFFFA000,
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(0x0000A000, vm.read_register_value(Register::R0));
    }

    #[test]
    fn or() {
        let program: [u32; 9] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 7 * 4),
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R2, 8 * 4),
            utils::create_instruction_two_registers(OpCode::LW, Register::R0, Register::R0),
            utils::create_instruction_two_registers(OpCode::LW, Register::R2, Register::R2),
            utils::create_instruction_two_registers(OpCode::OR, Register::R0, Register::R2),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            0x00000FFF,
            0xFFF00000,
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(0xFFF00FFF, vm.read_register_value(Register::R0));
    }

    #[test]
    fn xor() {
        let program: [u32; 9] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 7 * 4),
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R2, 8 * 4),
            utils::create_instruction_two_registers(OpCode::LW, Register::R0, Register::R0),
            utils::create_instruction_two_registers(OpCode::LW, Register::R2, Register::R2),
            utils::create_instruction_two_registers(OpCode::XOR, Register::R0, Register::R2),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            0x0000FFFF,
            0xFFFFF000,
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(0xFFFF0FFF, vm.read_register_value(Register::R0));
    }

    #[test]
    fn not() {
        let program: [u32; 10] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 8 * 4),
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R2, 9 * 4),
            utils::create_instruction_two_registers(OpCode::LW, Register::R0, Register::R0),
            utils::create_instruction_two_registers(OpCode::LW, Register::R2, Register::R2),
            utils::create_instruction_register(OpCode::NOT, Register::R0),
            utils::create_instruction_register(OpCode::NOT, Register::R2),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            0x00000000,
            0xFFFFFFF0,
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(0xFFFFFFFF, vm.read_register_value(Register::R0));
        assert_eq!(0x0000000F, vm.read_register_value(Register::R2));
    }

    #[test]
    fn srl() {
        let program: [u32; 9] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 7 * 4),
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R2, 8 * 4),
            utils::create_instruction_two_registers(OpCode::LW, Register::R0, Register::R0),
            utils::create_instruction_two_registers(OpCode::LW, Register::R2, Register::R2),
            utils::create_instruction_two_registers(OpCode::SRL, Register::R0, Register::R2),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            0x00FFFF00,
            4,
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(0x000FFFF0, vm.read_register_value(Register::R0));
    }

    #[test]
    fn sll() {
        let program: [u32; 9] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 7 * 4),
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R2, 8 * 4),
            utils::create_instruction_two_registers(OpCode::LW, Register::R0, Register::R0),
            utils::create_instruction_two_registers(OpCode::LW, Register::R2, Register::R2),
            utils::create_instruction_two_registers(OpCode::SLL, Register::R0, Register::R2),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            0x00FFFF00,
            4,
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(0x0FFFF000, vm.read_register_value(Register::R0));
    }

    #[test]
    fn srli() {
        let program: [u32; 6] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 5 * 4),
            utils::create_instruction_two_registers(OpCode::LW, Register::R0, Register::R0),
            utils::create_instruction_register_and_immediate(OpCode::SRLI, Register::R0, 4),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            0x00FFFF00,
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(0x000FFFF0, vm.read_register_value(Register::R0));
    }

    #[test]
    fn slli() {
        let program: [u32; 6] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 5 * 4),
            utils::create_instruction_two_registers(OpCode::LW, Register::R0, Register::R0),
            utils::create_instruction_register_and_immediate(OpCode::SLLI, Register::R0, 4),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            0x00FFFF00,
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Unexpected error!");
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(0x0FFFF000, vm.read_register_value(Register::R0));
    }

    #[test]
    fn new_with_program_overflow() {
        let program = vec!(0; BINARY_INTERPRETER_MEM_SIZE as usize + 100);
        let interpreter = BinaryInterpreter::new_with_program(&program);
        assert_eq!(None, interpreter, "Should be None");
    }

    #[test]
    fn new_with_initial() {
        let mem: Vec<u8> = vec!(1, 2, 3, 4, 10, 100);
        let interpreter = BinaryInterpreter::new_with_initial(&mem);
        assert_ne!(None, interpreter);
        let interpreter = interpreter.expect("Already checked");
        for i in 0..(mem.len() as u32) {
            let read_byte = interpreter.read_u8(i);
            assert_eq!(Some(mem[i as usize]), read_byte);
        }
    }

    #[test]
    fn new_with_initial_with_overflow() {
        let mem: Vec<u8> = vec!(0; BINARY_INTERPRETER_MEM_SIZE as usize + 100);
        let interpreter = BinaryInterpreter::new_with_initial(&mem);
        assert_eq!(None, interpreter);
    }

    #[test]
    fn read_u32_out_of_bounds() {
        let interpreter = BinaryInterpreter::new();
        assert_eq!(None, interpreter.read_u32(BINARY_INTERPRETER_MEM_SIZE as u32));
    }

    #[test]
    fn read_u16_out_of_bounds() {
        let interpreter = BinaryInterpreter::new();
        assert_eq!(None, interpreter.read_u16(BINARY_INTERPRETER_MEM_SIZE as u32));
    }

    #[test]
    fn read_u8_out_of_bounds() {
        let interpreter = BinaryInterpreter::new();
        assert_eq!(None, interpreter.read_u8(BINARY_INTERPRETER_MEM_SIZE as u32));
    }

    #[test]
    fn write_u32_out_of_bounds() {
        let mut interpreter = BinaryInterpreter::new();
        assert_eq!(false, interpreter.write_u32(BINARY_INTERPRETER_MEM_SIZE as u32, 0));
    }

    #[test]
    fn write_u16_out_of_bounds() {
        let mut interpreter = BinaryInterpreter::new();
        assert_eq!(false, interpreter.write_u16(BINARY_INTERPRETER_MEM_SIZE as u32, 0));
    }

    #[test]
    fn write_u8_out_of_bounds() {
        let mut interpreter = BinaryInterpreter::new();
        assert_eq!(false, interpreter.write_u8(BINARY_INTERPRETER_MEM_SIZE as u32, 0));
    }

    #[test]
    fn execute_out_of_bounds() {
        let interpreter = BinaryInterpreter::new();
        let mut vm = BinaryVirtualMachine::new(interpreter);
        vm.execute(BINARY_INTERPRETER_MEM_SIZE as u32);
        assert_eq!(Error::Memory as u32, vm.read_register_value(Register::ERR));
    }

    #[test]
    fn test_no_such_register() {
        let program: [u32; 1] = [utils::create_instruction_register(OpCode::J, Register::R0) + 0xF]; // Make sure to annihilate the register
        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Expected");
        let mut vm = BinaryVirtualMachine::new(interpreter);
        vm.execute_first();
        assert_eq!(Error::Register as u32, vm.read_register_value(Register::ERR));
    }

    #[test]
    fn test_no_such_register_write_register() {
        let program: [u32; 1] = [utils::create_instruction_two_registers(OpCode::CPY, Register::R0, Register::R1) + 0xE]; // Make sure to annihilate the register
        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Expected");
        let mut vm = BinaryVirtualMachine::new(interpreter);
        vm.execute_first();
        assert_eq!(Error::Register as u32, vm.read_register_value(Register::ERR));

        // test binary_register_operation_write0
        let program: [u32; 1] = [
            utils::create_instruction_two_registers(OpCode::CPY, Register::R0, Register::R1)
                + utils::create_instruction_two_registers(OpCode::CPY, Register::R1, Register::R0) * 0xF // sophisticated bs
        ]; // Make sure to annihilate the register
        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Expected");
        let mut vm = BinaryVirtualMachine::new(interpreter);
        vm.execute_first();
        assert_eq!(Error::Register as u32, vm.read_register_value(Register::ERR));
    }

    #[test]
    fn test_no_such_register_biop_write() {
        let program: [u32; 1] = [utils::create_instruction_two_registers(OpCode::ADD, Register::R0, Register::R1) + 0xE]; // Make sure to annihilate the register
        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Expected");
        let mut vm = BinaryVirtualMachine::new(interpreter);
        vm.execute_first();
        assert_eq!(Error::Register as u32, vm.read_register_value(Register::ERR));

        // test binary_register_operation_write0
        let program: [u32; 1] = [
            utils::create_instruction_two_registers(OpCode::ADD, Register::R0, Register::R1)
                + utils::create_instruction_two_registers(OpCode::CPY, Register::R1, Register::R0) * 0xF // sophisticated bs
        ]; // Make sure to annihilate the register
        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Expected");
        let mut vm = BinaryVirtualMachine::new(interpreter);
        vm.execute_first();
        assert_eq!(Error::Register as u32, vm.read_register_value(Register::ERR));

        let program: [u32; 1] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 123)
                + utils::create_instruction_two_registers(OpCode::CPY, Register::R1, Register::R0) * 0xF // sophisticated bs
        ]; // Make sure to annihilate the register
        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Expected");
        let mut vm = BinaryVirtualMachine::new(interpreter);
        vm.execute_first();
        assert_eq!(Error::Register as u32, vm.read_register_value(Register::ERR));

        // test binary_register_and_immediate_operation_write0
        let program: [u32; 1] = [
            utils::create_instruction_register_and_immediate(OpCode::SRLI, Register::R0, 123)
                + utils::create_instruction_two_registers(OpCode::CPY, Register::R1, Register::R0) * 0xF // sophisticated bs
        ]; // Make sure to annihilate the register
        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Expected");
        let mut vm = BinaryVirtualMachine::new(interpreter);
        vm.execute_first();
        assert_eq!(Error::Register as u32, vm.read_register_value(Register::ERR));
    }

    #[test]
    fn test_no_such_register_check_write_ip() {
        let program: [u32; 1] = [
            utils::create_instruction_register_and_immediate(OpCode::JGZI, Register::R0, 4)
                + utils::create_instruction_two_registers(OpCode::CPY, Register::R1, Register::R0) * 0xF // sophisticated bs
        ]; // Make sure to annihilate the register
        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Expected");
        let mut vm = BinaryVirtualMachine::new(interpreter);
        vm.execute_first();
        assert_eq!(Error::Register as u32, vm.read_register_value(Register::ERR));
    }

    #[test]
    fn test_no_such_register_not() {
        let program: [u32; 1] = [
            utils::create_instruction_register(OpCode::NOT, Register::R0) + 0xF
        ]; // Make sure to annihilate the register
        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Expected");
        let mut vm = BinaryVirtualMachine::new(interpreter);
        vm.execute_first();
        assert_eq!(Error::Register as u32, vm.read_register_value(Register::ERR));
    }

    #[test]
    fn test_cannot_write_register() {
        let program: [u32; 1] = [utils::create_instruction_two_registers(OpCode::CPY, Register::IP, Register::R0)];
        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Expected");
        let mut vm = BinaryVirtualMachine::new(interpreter);
        vm.execute_first();
        assert_eq!(Error::ReadonlyRegister as u32, vm.read_register_value(Register::ERR));

        let program: [u32; 1] = [utils::create_instruction_two_registers(OpCode::CPY, Register::ERR, Register::R0)];
        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Expected");
        let mut vm = BinaryVirtualMachine::new(interpreter);
        vm.execute_first();
        assert_eq!(Error::ReadonlyRegister as u32, vm.read_register_value(Register::ERR));
    }

    #[test]
    fn test_can_write_registers() {
        for register in [Register::R0, Register::R1, Register::R2, Register::R3, Register::R4, Register::R5, Register::R6, Register::R7, Register::RA, Register::SP] {
            let program: [u32; 3] = [
                utils::create_instruction_two_registers(OpCode::CPY, register, Register::IP),
                LOAD_0_IN_R1_INSTRUCTION,
                SYSCALLI_EXIT_INSTRUCTION
            ];
            let interpreter = BinaryInterpreter::new_with_program(&program).expect("Expected");
            let mut vm = BinaryVirtualMachine::new(interpreter);
            vm.execute_first();
            assert_eq!(Error::NoError as u32, vm.read_register_value(Register::ERR));
        }
    }

    #[test]
    fn test_invalid_syscall() {
        let program: [u32; 1] = [utils::create_instruction_immediate(OpCode::SYSCALLI, 0xFF)];
        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Expected");
        let mut vm = BinaryVirtualMachine::new(interpreter);
        vm.execute_first();
        assert_eq!(Error::Syscall as u32, vm.read_register_value(Register::ERR));
    }

    #[test]
    fn lw_edge() {
        let program: [u32; 5] = [
            utils::create_instruction_register_and_immediate(OpCode::LWI, Register::R0, 4 * 4),
            utils::create_instruction_two_registers(OpCode::LW, Register::R0, Register::R0),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            BINARY_INTERPRETER_MEM_SIZE - 4
        ];
        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Expected");
        let mut vm = BinaryVirtualMachine::new(interpreter);
        vm.execute_first();
        assert_eq!(Error::NoError as u32, vm.read_register_value(Register::ERR));
    }

    #[test]
    fn lh_edge() {
        let program: [u32; 5] = [
            utils::create_instruction_register_and_immediate(OpCode::LWI, Register::R0, 4 * 4),
            utils::create_instruction_two_registers(OpCode::LH, Register::R0, Register::R0),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            BINARY_INTERPRETER_MEM_SIZE - 2
        ];
        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Expected");
        let mut vm = BinaryVirtualMachine::new(interpreter);
        vm.execute_first();
        assert_eq!(Error::NoError as u32, vm.read_register_value(Register::ERR));
    }

    #[test]
    fn lb_edge() {
        let program: [u32; 5] = [
            utils::create_instruction_register_and_immediate(OpCode::LWI, Register::R0, 4 * 4),
            utils::create_instruction_two_registers(OpCode::LB, Register::R0, Register::R0),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            BINARY_INTERPRETER_MEM_SIZE - 1
        ];
        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Expected");
        let mut vm = BinaryVirtualMachine::new(interpreter);
        vm.execute_first();
        assert_eq!(Error::NoError as u32, vm.read_register_value(Register::ERR));
    }

    #[test]
    fn lw_out_of_bounds() {
        for i in 0..3 { // 32-bit = 4-byte
            let program: [u32; 2] = [
                utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, BINARY_INTERPRETER_MEM_SIZE as u32 - i),
                utils::create_instruction_two_registers(OpCode::LW, Register::R0, Register::R0)
            ];
            let interpreter = BinaryInterpreter::new_with_program(&program).expect("Expected");
            let mut vm = BinaryVirtualMachine::new(interpreter);
            vm.execute_first();
            assert_eq!(Error::Memory as u32, vm.read_register_value(Register::ERR));
        }
    }

    #[test]
    fn lh_out_of_bounds() {
        for i in 0..1 { // 16-bit = 2-byte
            let program: [u32; 2] = [
                utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, BINARY_INTERPRETER_MEM_SIZE as u32 - i),
                utils::create_instruction_two_registers(OpCode::LH, Register::R0, Register::R0)
            ];
            let interpreter = BinaryInterpreter::new_with_program(&program).expect("Expected");
            let mut vm = BinaryVirtualMachine::new(interpreter);
            vm.execute_first();
            assert_eq!(Error::Memory as u32, vm.read_register_value(Register::ERR));
        }
    }

    #[test]
    fn lb_out_of_bounds() {
        let program: [u32; 2] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, BINARY_INTERPRETER_MEM_SIZE as u32),
            utils::create_instruction_two_registers(OpCode::LB, Register::R0, Register::R0)
        ];
        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Expected");
        let mut vm = BinaryVirtualMachine::new(interpreter);
        vm.execute_first();
        assert_eq!(Error::Memory as u32, vm.read_register_value(Register::ERR));
    }

    #[test]
    fn edge_binary_interpreter() {
        let mut interpreter = BinaryInterpreter::new();
        assert_eq!(Some(0), interpreter.read_u8(BINARY_INTERPRETER_MEM_SIZE - 1));
        assert_eq!(true, interpreter.write_u8(BINARY_INTERPRETER_MEM_SIZE - 1, 128 as u8));
        assert_eq!(Some(128), interpreter.read_u8(BINARY_INTERPRETER_MEM_SIZE - 1));
        
        let mut interpreter = BinaryInterpreter::new();
        assert_eq!(Some(0), interpreter.read_u16(BINARY_INTERPRETER_MEM_SIZE - 2));
        assert_eq!(true, interpreter.write_u16(BINARY_INTERPRETER_MEM_SIZE - 2, 30230));
        assert_eq!(Some(30230), interpreter.read_u16(BINARY_INTERPRETER_MEM_SIZE - 2));

        let mut interpreter = BinaryInterpreter::new();
        assert_eq!(Some(0), interpreter.read_u32(BINARY_INTERPRETER_MEM_SIZE - 4));
        assert_eq!(true, interpreter.write_u32(BINARY_INTERPRETER_MEM_SIZE - 4, 30230));
        assert_eq!(Some(30230), interpreter.read_u32(BINARY_INTERPRETER_MEM_SIZE - 4));
    }

    #[test]
    fn sw_edge() {
        let program: [u32; 5] = [
            utils::create_instruction_register_and_immediate(OpCode::LWI, Register::R0, 4 * 4),
            utils::create_instruction_two_registers(OpCode::SW, Register::R0, Register::R0),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            BINARY_INTERPRETER_MEM_SIZE - 4
        ];
        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Expected");
        let mut vm = BinaryVirtualMachine::new(interpreter);
        vm.execute_first();
        assert_eq!(Error::NoError as u32, vm.read_register_value(Register::ERR));
    }

    #[test]
    fn sh_edge() {
        let program: [u32; 5] = [
            utils::create_instruction_register_and_immediate(OpCode::LWI, Register::R0, 4 * 4),
            utils::create_instruction_two_registers(OpCode::SH, Register::R0, Register::R0),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            BINARY_INTERPRETER_MEM_SIZE - 2
        ];
        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Expected");
        let mut vm = BinaryVirtualMachine::new(interpreter);
        vm.execute_first();
        assert_eq!(Error::NoError as u32, vm.read_register_value(Register::ERR));
    }

    #[test]
    fn sb_edge() {
        let program: [u32; 5] = [
            utils::create_instruction_register_and_immediate(OpCode::LWI, Register::R0, 4 * 4),
            utils::create_instruction_two_registers(OpCode::SB, Register::R0, Register::R0),
            LOAD_0_IN_R1_INSTRUCTION,
            SYSCALLI_EXIT_INSTRUCTION,
            BINARY_INTERPRETER_MEM_SIZE - 1
        ];
        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Expected");
        let mut vm = BinaryVirtualMachine::new(interpreter);
        vm.execute_first();
        assert_eq!(Error::NoError as u32, vm.read_register_value(Register::ERR));
    }

    #[test]
    fn sw_out_of_bounds() {
        for i in 0..3 { // 32-bit = 4-byte
            let program: [u32; 2] = [
                utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, BINARY_INTERPRETER_MEM_SIZE as u32 - i),
                utils::create_instruction_two_registers(OpCode::SW, Register::R0, Register::R0)
            ];
            let interpreter = BinaryInterpreter::new_with_program(&program).expect("Expected");
            let mut vm = BinaryVirtualMachine::new(interpreter);
            vm.execute_first();
            assert_eq!(Error::Memory as u32, vm.read_register_value(Register::ERR));
        }
    }

    #[test]
    fn sh_out_of_bounds() {
        for i in 0..1 { // 16-bit = 2-byte
            let program: [u32; 2] = [
                utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, BINARY_INTERPRETER_MEM_SIZE as u32 - i),
                utils::create_instruction_two_registers(OpCode::SH, Register::R0, Register::R0)
            ];
            let interpreter = BinaryInterpreter::new_with_program(&program).expect("Expected");
            let mut vm = BinaryVirtualMachine::new(interpreter);
            vm.execute_first();
            assert_eq!(Error::Memory as u32, vm.read_register_value(Register::ERR));
        }
    }

    #[test]
    fn sb_out_of_bounds() {
        let program: [u32; 2] = [
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, BINARY_INTERPRETER_MEM_SIZE as u32),
            utils::create_instruction_two_registers(OpCode::SB, Register::R0, Register::R0)
        ];
        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Expected");
        let mut vm = BinaryVirtualMachine::new(interpreter);
        vm.execute_first();
        assert_eq!(Error::Memory as u32, vm.read_register_value(Register::ERR));
    }

    #[test]
    fn test_no_such_instruction() {
        let program: [u32; 1] = [
            0xFF000000
        ];
        let interpreter = BinaryInterpreter::new_with_program(&program).expect("Expected");
        let mut vm = BinaryVirtualMachine::new(interpreter);
        vm.execute_first();
        assert_eq!(Error::OpCode as u32, vm.read_register_value(Register::ERR));
    }

    #[test]
    fn test_for_error() {
        assert_eq!(Error::NoError, Error::NoError);
        assert_eq!(Error::ReadonlyRegister, Error::ReadonlyRegister);
        assert_ne!(Error::ReadonlyRegister, Error::NoError);
    }
}
