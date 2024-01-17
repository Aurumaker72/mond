use mond::generator::{Instruction, Register};
use mond::interpreter::Interpreter;

#[test]
pub fn addition() {
    let mut interpreter = Interpreter::default();
    interpreter.execute(vec![
        Instruction::Init(Register::GPR0, 0),
        Instruction::Init(Register::GPR1, 7),
        Instruction::Init(Register::GPR2, 3),
        Instruction::Add(Register::GPR0, Register::GPR1, Register::GPR2),
    ]);
    assert_eq!(interpreter.registers[&Register::GPR0], 10)
}

#[test]
pub fn subtraction() {
    let mut interpreter = Interpreter::default();
    interpreter.execute(vec![
        Instruction::Init(Register::GPR0, 0),
        Instruction::Init(Register::GPR1, 7),
        Instruction::Init(Register::GPR2, 3),
        Instruction::Sub(Register::GPR0, Register::GPR1, Register::GPR2),
    ]);
    assert_eq!(interpreter.registers[&Register::GPR0], 4)
}

#[test]
pub fn multiply() {
    let mut interpreter = Interpreter::default();
    interpreter.execute(vec![
        Instruction::Init(Register::GPR0, 0),
        Instruction::Init(Register::GPR1, 7),
        Instruction::Init(Register::GPR2, 3),
        Instruction::Mul(Register::GPR0, Register::GPR1, Register::GPR2),
    ]);
    assert_eq!(interpreter.registers[&Register::GPR0], 21)
}

#[test]
pub fn divide() {
    let mut interpreter = Interpreter::default();
    interpreter.execute(vec![
        Instruction::Init(Register::GPR0, 0),
        Instruction::Init(Register::GPR1, 8),
        Instruction::Init(Register::GPR2, 2),
        Instruction::Div(Register::GPR0, Register::GPR1, Register::GPR2),
    ]);
    assert_eq!(interpreter.registers[&Register::GPR0], 4)
}