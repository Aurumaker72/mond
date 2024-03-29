use crate::interpreter::Instruction;

pub mod tokenizer;
pub mod generator;
pub mod interpreter;


pub type Program = Vec<Instruction>;
