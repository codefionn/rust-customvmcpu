mod runtime;

use runtime::{Interpreter, BinaryVirtualMachine, OpCode, BinaryInterpreter};

fn main() {
    let interpreter = BinaryInterpreter::new_with_program(&[(OpCode::SYSCALLI as u32) << 3 * 8]);
    let mut vm = BinaryVirtualMachine::new(interpreter);
    
    vm.execute_first();
}
