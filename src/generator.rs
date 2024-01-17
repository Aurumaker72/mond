use crate::tokenizer::Token;
use crate::tokenizer::Token::{Identifier, Keyword};
use crate::Block;

#[derive(Clone, Debug, Hash)]
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
    Init(Register, u64),

    // Jumps to the specified address
    Jmp(u64),
    // Returns to the address of the last jmp instruction
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

pub fn generate(token_bag: Vec<Vec<Token>>) -> Vec<Block> {
    let mut blocks = vec![];

    for tokens in token_bag {
        if tokens[0] == Keyword("let".to_string()) {
            let Identifier(identifier) = tokens[1].clone() else {
                todo!()
            };
        }
    }

    blocks
}
