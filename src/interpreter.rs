use crate::generator::{Instruction, Register};
use crate::Program;
use std::collections::HashMap;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Interpreter {
    pub registers: HashMap<Register, i64>,
    pub address: u64,
    pub callstack: Vec<u64>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut interpreter = Interpreter {
            registers: Default::default(),
            address: 0,
            callstack: vec![],
        };

        for register in Register::iter() {
            interpreter.registers.insert(register, 0);
        }

        interpreter
    }

    pub fn execute(&mut self, program: Program) {
        loop {
            if self.address >= program.len() as u64 {
                break;
            }
            let instruction = program[self.address as usize].clone();

            match instruction {
                Instruction::Copy(dst, src) => {
                    self.registers.insert(dst, self.registers[&src]);
                }
                Instruction::Init(dst, val) => {
                    self.registers.insert(dst, val);
                }
                Instruction::Jmp(addr) => {
                    self.address = addr;
                    self.callstack.push(self.address);
                }
                Instruction::Jg(addr) => {
                    if self.registers[&Register::GPR0] > self.registers[&Register::GPR1] {
                        self.address = addr;
                    }
                }
                Instruction::Jge(addr) => {
                    if self.registers[&Register::GPR0] >= self.registers[&Register::GPR1] {
                        self.address = addr;
                    }
                }
                Instruction::Jl(addr) => {
                    if self.registers[&Register::GPR0] < self.registers[&Register::GPR1] {
                        self.address = addr;
                    }
                }
                Instruction::Jle(addr) => {
                    if self.registers[&Register::GPR0] <= self.registers[&Register::GPR1] {
                        self.address = addr;
                    }
                }
                Instruction::Jeq(addr) => {
                    if self.registers[&Register::GPR0] == self.registers[&Register::GPR1] {
                        self.address = addr;
                    }
                }
                Instruction::Ret => {
                    if self.callstack.is_empty() {
                        break;
                    }
                    self.address = self.callstack.pop().unwrap();
                }
                Instruction::Addr => {
                    self.registers.insert(Register::GPR0, self.address as i64);
                }
                Instruction::Add(out, reg1, reg2) => {
                    self.registers
                        .insert(out, self.registers[&reg1] + self.registers[&reg2]);
                }
                Instruction::Sub(out, reg1, reg2) => {
                    self.registers
                        .insert(out, self.registers[&reg1] - self.registers[&reg2]);
                }
                Instruction::Mul(out, reg1, reg2) => {
                    self.registers
                        .insert(out, self.registers[&reg1] * self.registers[&reg2]);
                }
                Instruction::Div(out, reg1, reg2) => {
                    self.registers
                        .insert(out, self.registers[&reg1] / self.registers[&reg2]);
                }
            }

            self.address += 1;
        }

        println!("Execution terminated");
        println!("{:?}", self);
    }
}
