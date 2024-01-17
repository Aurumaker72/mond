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
            Instruction::Copy(dst, src) => {}
            Instruction::Init(dst, val) => {}
            Instruction::Jmp(addr) => {}
            Instruction::Ret => {}
            Instruction::Addr => {}
            Instruction::Add(_, _, _) => {}
            Instruction::Sub(_, _, _) => {}
            Instruction::Mul(_, _, _) => {}
            Instruction::Div(_, _, _) => {}
        }

        interpreter.address += 1;
    }
}

