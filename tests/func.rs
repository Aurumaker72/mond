use mond::generator::{Instruction, Register};
use mond::interpreter::Interpreter;

#[test]
pub fn func() {
    let mut interpreter = Interpreter::new();
    interpreter.execute(vec![
        // Set up argument
        Instruction::Init(Register::GPR1, 5),
        // Jump into the routine
        Instruction::Jmp(2),
        // Done, exit
        Instruction::Ret,

        // Routine: Multiplies GPR1 by 2 into GPR0
        // Clobbers GPR2, jumps back to caller
        Instruction::Init(Register::GPR2, 2),
        Instruction::Mul(Register::GPR0, Register::GPR1, Register::GPR2),
        Instruction::Ret

    ]);
}