use crate::Program;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;


#[derive(Clone, Debug, Hash, Eq, PartialEq, EnumIter)]
pub enum Register {
    GPR0,
    GPR1,
    GPR2,
    GPR3,
    GPR4,
    GPR5,
    GPR6,
    GPR7,
    GPR8,
    GPR9,
}

#[derive(Clone, Debug)]
pub enum Instruction {
    // Copies register contents
    Copy(Register, Register),
    // Initializes a register with the specified value
    Init(Register, i64),

    // Jumps to the specified address, pushing the address to the callstack
    Jmp(u64),
    // Jumps to the specified address if GPR0 is greater than GPR1
    Jg(u64),
    // Jumps to the specified address if GPR0 is greater than or equal to GPR1
    Jge(u64),
    // Jumps to the specified address if GPR0 is less than GPR1
    Jl(u64),
    // Jumps to the specified address if GPR0 is less than or equal to GPR1
    Jle(u64),
    // Jumps to the specified address if GPR0 is equal to GPR1
    Jeq(u64),
    // Returns to the address of the last jmp instruction
    // If called with empty callstack, exits the program
    Ret,
    // Copies the current address into GPR0
    Addr,

    // Adds and stores the output in the first register
    Add(Register, Register, Register),
    // Subtracts and stores the output in the first register
    Sub(Register, Register, Register),
    // Multiplies and stores the output in the first register
    Mul(Register, Register, Register),
    // Divides and stores the output in the first register
    Div(Register, Register, Register),
}

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
