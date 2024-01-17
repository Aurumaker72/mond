use crate::generator::{Instruction, Register};
use crate::Program;
use std::collections::HashMap;
use std::ptr::addr_of;

#[derive(Default, Debug)]
pub struct Interpreter {
    pub registers: HashMap<Register, i64>,
    pub address: u64,
}

impl Interpreter {
    pub fn execute(&mut self, program: Program) {
        loop {
            if self.address >= program.len() as u64 {
                println!("Program finished");
                println!("{:?}", self);
                return;
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
                    self.address = addr - 1;
                }
                Instruction::Jg(addr) => {
                    if self.registers[&Register::GPR0] > self.registers[&Register::GPR1] {
                        self.address = addr - 1;
                    }
                }
                Instruction::Jge(addr) => {
                    if self.registers[&Register::GPR0] >= self.registers[&Register::GPR1] {
                        self.address = addr - 1;
                    }
                }
                Instruction::Jl(addr) => {
                    if self.registers[&Register::GPR0] < self.registers[&Register::GPR1] {
                        self.address = addr - 1;
                    }
                }
                Instruction::Jle(addr) => {
                    if self.registers[&Register::GPR0] <= self.registers[&Register::GPR1] {
                        self.address = addr - 1;
                    }
                }
                Instruction::Jeq(addr) => {
                    if self.registers[&Register::GPR0] == self.registers[&Register::GPR1] {
                        self.address = addr - 1;
                    }
                }
                Instruction::Ret => {
                    todo!()
                }
                Instruction::Addr => {
                    self.registers.insert(Register::GPR0, self.address as i64);
                }
                Instruction::Add(out, reg1, reg2) => {
                    self.registers.insert(out, self.registers[&reg1] + self.registers[&reg2]);
                }
                Instruction::Sub(out, reg1, reg2) => {
                    self.registers.insert(out, self.registers[&reg1] - self.registers[&reg2]);
                }
                Instruction::Mul(out, reg1, reg2) => {
                    self.registers.insert(out, self.registers[&reg1] * self.registers[&reg2]);
                }
                Instruction::Div(out, reg1, reg2) => {
                    self.registers.insert(out, self.registers[&reg1] / self.registers[&reg2]);
                }

            }

            self.address += 1;
        }
    }
}
