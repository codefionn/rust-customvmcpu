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

pub const BINARY_INTERPRETER_MEM_SIZE: usize = 1024 * 16 * 4;

pub struct BinaryInterpreter {
    memory: [u8; BINARY_INTERPRETER_MEM_SIZE],
}

impl BinaryInterpreter {
    pub fn new() -> BinaryInterpreter {
        BinaryInterpreter { memory: [0; BINARY_INTERPRETER_MEM_SIZE] }
    }

    pub fn new_with_program(program: &[u32]) -> BinaryInterpreter {
        let mut result = Self::new();
        if program.len() > result.memory.len() {
            panic!("Program length must be smaller than memory");
        }

        let start_pos: u32 = 0;
        for pos in 0..program.len() {
            if !result.write_u32(pos as u32 * 4 + start_pos, program[pos]) {
                panic!("Program length must be smaller than memory");
            }
        }

        result
    }

    /// Initializes BinaryInterpreter with initial memory
    pub fn new_with_initial(initial_memory: &Vec<u8>) -> BinaryInterpreter {
        let mut result = Self::new();
        result.memory[0..initial_memory.len()].copy_from_slice(&initial_memory);
        result
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

            self.write_register_value(Register::IP, self.read_register_value(Register::IP) + 4);
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
                    let (reg0, reg1) = Self::get_two_registers(instruction);
                    if let (Some(reg_value0), Some(reg_value1)) = (Register::from_u8(reg0), Register::from_u8(reg1)) {
                        self.write_user_register_value(reg_value0, self.read_register_value(reg_value1));
                    }
                    else {
                        eprintln!("Register {:?} or {:?} does not exists!", reg0, reg1);
                        self.write_error(Error::Register);
                    }
                },
                // Load-store
                OpCode::LW => {
                    let (reg0, reg1) = Self::get_two_registers(instruction);
                    if let (Some(reg_value0), Some(reg_value1)) = (Register::from_u8(reg0), Register::from_u8(reg1)) {
                        if let Some(result) = self.interpreter.read_u32(self.read_user_register_value(reg_value1)) {
                            self.write_user_register_value(reg_value0, result);
                        }
                        else {
                            self.write_error(Error::Memory);
                        }
                    }
                    else {
                        eprintln!("Register {:?} or {:?} does not exists!", reg0, reg1);
                        self.write_error(Error::Register);
                    }
                },
                OpCode::SW => {
                    let (reg0, reg1) = Self::get_two_registers(instruction);
                    if let (Some(reg_value0), Some(reg_value1)) = (Register::from_u8(reg0), Register::from_u8(reg1)) {
                        if !self.interpreter.write_u32(self.read_user_register_value(reg_value1), self.read_user_register_value(reg_value0)) {
                            self.write_error(Error::Memory);
                        }
                    }
                    else {
                        eprintln!("Register {:?} or {:?} does not exists!", reg0, reg1);
                        self.write_error(Error::Register);
                    }
                },
                OpCode::LH => {
                    let (reg0, reg1) = Self::get_two_registers(instruction);
                    if let (Some(reg_value0), Some(reg_value1)) = (Register::from_u8(reg0), Register::from_u8(reg1)) {
                        if let Some(result) = self.interpreter.read_u16(self.read_user_register_value(reg_value1)) {
                            self.write_user_register_value(reg_value0, result as u32);
                        }
                        else {
                            self.write_error(Error::Memory);
                        }
                    }
                    else {
                        eprintln!("Register {:?} or {:?} does not exists!", reg0, reg1);
                        self.write_error(Error::Register);
                    }
                },
                OpCode::SH => {
                    let (reg0, reg1) = Self::get_two_registers(instruction);
                    if let (Some(reg_value0), Some(reg_value1)) = (Register::from_u8(reg0), Register::from_u8(reg1)) {
                        if !self.interpreter.write_u16(self.read_user_register_value(reg_value1), (self.read_user_register_value(reg_value0) & 0x0000FFFF).try_into().expect("Unexpected error")) {
                            self.write_error(Error::Memory);
                        }
                    }
                    else {
                        eprintln!("Register {:?} or {:?} does not exists!", reg0, reg1);
                        self.write_error(Error::Register);
                    }
                },
                OpCode::LB => {
                    let (reg0, reg1) = Self::get_two_registers(instruction);
                    if let (Some(reg_value0), Some(reg_value1)) = (Register::from_u8(reg0), Register::from_u8(reg1)) {
                        if let Some(result) = self.interpreter.read_u8(self.read_user_register_value(reg_value1)) {
                            self.write_user_register_value(reg_value0, result as u32);
                        }
                        else {
                            self.write_error(Error::Memory);
                        }
                    }
                    else {
                        eprintln!("Register {:?} or {:?} does not exists!", reg0, reg1);
                        self.write_error(Error::Register);
                    }
                },
                OpCode::SB => {
                    let (reg0, reg1) = Self::get_two_registers(instruction);
                    if let (Some(reg_value0), Some(reg_value1)) = (Register::from_u8(reg0), Register::from_u8(reg1)) {
                        if !self.interpreter.write_u8(self.read_user_register_value(reg_value1), (self.read_user_register_value(reg_value0) & 0x000000FF).try_into().expect("Unexpected error")) {
                            self.write_error(Error::Memory);
                        }
                    }
                    else {
                        eprintln!("Register {:?} or {:?} does not exists!", reg0, reg1);
                        self.write_error(Error::Register);
                    }
                },
                OpCode::LI => {
                    let (reg0, imm1) = Self::get_register_and_immediate(instruction);
                    if let Some(reg_value0) = Register::from_u8(reg0) {
                        self.write_user_register_value(reg_value0, imm1);
                    }
                    else {
                        eprintln!("Register {:?} does not exists!", reg0);
                        self.write_error(Error::Register);
                    }
                },
                // Arithmetics
                OpCode::ADD => {
                    let (reg0, reg1) = Self::get_two_registers(instruction);
                    if let (Some(reg_value0), Some(reg_value1)) = (Register::from_u8(reg0), Register::from_u8(reg1)) {
                        self.write_user_register_value(reg_value0, self.read_user_register_value(reg_value0) + self.read_user_register_value(reg_value1));
                    }
                    else {
                        eprintln!("Register {:?} or {:?} does not exists!", reg0, reg1);
                        self.write_error(Error::Register);
                    }
                },
                OpCode::SUB => {
                    let (reg0, reg1) = Self::get_two_registers(instruction);
                    if let (Some(reg_value0), Some(reg_value1)) = (Register::from_u8(reg0), Register::from_u8(reg1)) {
                        self.write_user_register_value(reg_value0, self.read_user_register_value(reg_value0) - self.read_user_register_value(reg_value1));
                    }
                    else {
                        eprintln!("Register {:?} or {:?} does not exists!", reg0, reg1);
                        self.write_error(Error::Register);
                    }
                },
                OpCode::MUL => {
                    let (reg0, reg1) = Self::get_two_registers(instruction);
                    if let (Some(reg_value0), Some(reg_value1)) = (Register::from_u8(reg0), Register::from_u8(reg1)) {
                        self.write_user_register_value(reg_value0, self.read_user_register_value(reg_value0) * self.read_user_register_value(reg_value1));
                    }
                    else {
                        eprintln!("Register {:?} or {:?} does not exists!", reg0, reg1);
                        self.write_error(Error::Register);
                    }
                },
                OpCode::DIV => {
                    let (reg0, reg1) = Self::get_two_registers(instruction);
                    if let (Some(reg_value0), Some(reg_value1)) = (Register::from_u8(reg0), Register::from_u8(reg1)) {
                        let divisor = self.read_user_register_value(reg_value1);
                        if divisor == 0 {
                            self.write_error(Error::DivisorNotZero);
                            self.write_register_value(reg_value0, 0);
                        } else {
                            self.write_user_register_value(reg_value0, self.read_user_register_value(reg_value0) / divisor);
                        }
                    }
                    else {
                        eprintln!("Register {:?} or {:?} does not exists!", reg0, reg1);
                        self.write_error(Error::Register);
                    }
                },
                // Unconditional jumps
                OpCode::J => {
                    let reg = Self::get_registers(instruction);
                    if let Some(reg_value) = Register::from_u8(reg) {
                        let address = self.read_user_register_value(reg_value);
                        self.write_register_value(Register::IP, address - 4); // Minus 4 because this will be added after every cycle
                    }
                    else {
                        eprintln!("Register {:?} does not exists!", reg);
                        self.write_error(Error::Register);
                    }
                },
                OpCode::JI => {
                    let address = Self::get_immediate(instruction);
                    self.write_register_value(Register::IP, address - 4); // Minus 4 because this will be added after every cycle
                }
                OpCode::JIL => {
                    let address = Self::get_immediate(instruction);
                    self.write_register_value(Register::RA, self.read_register_value(Register::IP) + 4); // Plus 4 because it points to the next instruction
                    self.write_register_value(Register::IP, address - 4); // Minus 4 because this will be added after every cycle
                }
                _ => {
                    eprintln!("Instruction {:?} does not exist!", opcode);
                    self.write_error(Error::OpCode);
                }
            }
        }
        else {
            eprintln!("Instruction {:?} does not exist!", opcode);
            self.write_error(Error::OpCode);
            return;
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
            Register::IP | Register::ERR | Register::RA => true,
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
    fn get_two_register_and_immediate(instruction: u32) -> (u8, u8, u32) {
        (
            u8::try_from((instruction & 0x00F00000) >> (2 * 8 + 4)).expect("Unexpected failure!"),
            u8::try_from((instruction & 0x000F0000) >> (2 * 8 + 0)).expect("Unexpected failure!"),
            instruction & 0x0000FFFF
        )
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
    use super::{OpCode, BinaryInterpreter, BinaryVirtualMachine, Interpreter, Register, utils, Error, ERROR_START_NUM};

    const SYSCALLI_EXIT_INSTRUCTION: u32 = u32::to_le((OpCode::SYSCALLI as u32) << 3 * 8);
    const LOAD_0_IN_R1_INSTRUCTION: u32 = utils::create_instruction_register_and_immediate(OpCode::LI, Register::R1, 0);

    #[test]
    fn get_opcode() {
        assert_eq!(OpCode::SYSCALLI as u8, BinaryVirtualMachine::get_opcode(SYSCALLI_EXIT_INSTRUCTION));
    }

    #[test]
    fn syscall_exit() {
        let syscode_inst = SYSCALLI_EXIT_INSTRUCTION;
        let interpreter = BinaryInterpreter::new_with_program(&[syscode_inst]);
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.read_register_value(Register::IP));
        assert_eq!(Some(syscode_inst), vm.get_interpreter().read_u32(0));
        assert_eq!(0, vm.execute_first());
        assert_eq!(0, vm.read_register_value(Register::IP));

        let syscode_inst = SYSCALLI_EXIT_INSTRUCTION;
        let interpreter = BinaryInterpreter::new_with_program(&[
            syscode_inst,
            utils::create_instruction_register_and_immediate(OpCode::LI, Register::R0, 32)
        ]);
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
        let interpreter = BinaryInterpreter::new_with_program(&[inst, LOAD_0_IN_R1_INSTRUCTION, SYSCALLI_EXIT_INSTRUCTION]);
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(564, vm.read_register_value(Register::R0));
    }

    #[test]
    fn li_r1() {
        let inst = utils::create_instruction_register_and_immediate(OpCode::LI, Register::R1, 563);
        let interpreter = BinaryInterpreter::new_with_program(&[inst, SYSCALLI_EXIT_INSTRUCTION]);
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(563, vm.execute_first());
        assert_eq!(563, vm.read_register_value(Register::R1));
    }

    #[test]
    fn li_r7() {
        let inst = utils::create_instruction_register_and_immediate(OpCode::LI, Register::R7, 513);
        let interpreter = BinaryInterpreter::new_with_program(&[inst, LOAD_0_IN_R1_INSTRUCTION, SYSCALLI_EXIT_INSTRUCTION]);
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(513, vm.read_register_value(Register::R7));
    }

    #[test]
    fn li_ip() {
        let inst = utils::create_instruction_register_and_immediate(OpCode::LI, Register::IP, 12);
        let interpreter = BinaryInterpreter::new_with_program(&[inst, LOAD_0_IN_R1_INSTRUCTION, SYSCALLI_EXIT_INSTRUCTION]);
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(ERROR_START_NUM + (Error::ReadonlyRegister as u32), vm.execute_first());
    }

    #[test]
    fn li_err() {
        let inst = utils::create_instruction_register_and_immediate(OpCode::LI, Register::ERR, 12);
        let interpreter = BinaryInterpreter::new_with_program(&[inst, LOAD_0_IN_R1_INSTRUCTION, SYSCALLI_EXIT_INSTRUCTION]);
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

        let interpreter = BinaryInterpreter::new_with_program(&program);
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

        let interpreter = BinaryInterpreter::new_with_program(&program);
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

        let interpreter = BinaryInterpreter::new_with_program(&program);
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

        let interpreter = BinaryInterpreter::new_with_program(&program);
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

        let interpreter = BinaryInterpreter::new_with_program(&program);
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

        let interpreter = BinaryInterpreter::new_with_program(&program);
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

      let interpreter = BinaryInterpreter::new_with_program(&program);
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
            1032
        ];

        let interpreter = BinaryInterpreter::new_with_program(&program);
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(1032, vm.read_register_value(Register::R0));
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

        let interpreter = BinaryInterpreter::new_with_program(&program);
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

        let interpreter = BinaryInterpreter::new_with_program(&program);
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

        let interpreter = BinaryInterpreter::new_with_program(&program);
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

        let interpreter = BinaryInterpreter::new_with_program(&program);
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

        let interpreter = BinaryInterpreter::new_with_program(&program);
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

        let interpreter = BinaryInterpreter::new_with_program(&program);
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

        let interpreter = BinaryInterpreter::new_with_program(&program);
        let mut vm = BinaryVirtualMachine::new(interpreter);

        assert_eq!(0, vm.execute_first());
        assert_eq!(234, vm.get_interpreter().read_u8(5 * 4).expect("Cannot read memory address"));
        assert_eq!(234, vm.get_interpreter().read_u32(5 * 4).expect("Cannot read memory address"));
    }
}
