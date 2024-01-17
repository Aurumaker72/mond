use crate::generator::{Instruction, Register};
use crate::Program;
use std::collections::HashMap;
use std::ptr::addr_of;

#[derive(Default, Debug)]
pub struct Interpreter {
    registers: HashMap<Register, u64>,
    address: u64,
}

pub fn run(program: Program) {
    let mut interpreter = Interpreter::default();

    loop {
        if interpreter.address >= program.len() as u64 {
            println!("Program finished");
            println!("{:?}", interpreter);
            return;
        }
        let instruction = program[interpreter.address as usize].clone();

        match instruction {
            Instruction::Copy(dst, src) => {
                interpreter.registers.insert(dst, interpreter.registers[&src]);
            }
            Instruction::Init(dst, val) => {
                interpreter.registers.insert(dst, val);
            }
            Instruction::Jmp(addr) => {
                interpreter.address = addr - 1;
            }
            Instruction::Ret => {
                todo!()
            }
            Instruction::Addr => {
                interpreter.registers.insert(Register::GPR0, interpreter.address);
            }
            Instruction::Add(out, reg1, reg2) => {
                interpreter.registers.insert(out, interpreter.registers[&reg1] + interpreter.registers[&reg2]);
            }
            Instruction::Sub(out, reg1, reg2) => {
                interpreter.registers.insert(out, interpreter.registers[&reg1] - interpreter.registers[&reg2]);
            }
            Instruction::Mul(out, reg1, reg2) => {
                interpreter.registers.insert(out, interpreter.registers[&reg1] * interpreter.registers[&reg2]);
            }
            Instruction::Div(out, reg1, reg2) => {
                interpreter.registers.insert(out, interpreter.registers[&reg1] / interpreter.registers[&reg2]);
            }
        }

        interpreter.address += 1;
    }
}

