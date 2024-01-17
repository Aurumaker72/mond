use crate::generator::Instruction;

pub mod tokenizer;
pub mod generator;
pub mod interpreter;


pub type Block = Vec<Instruction>;
