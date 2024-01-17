use crate::generator::Register;
use crate::Block;
use std::collections::HashMap;

#[derive(Default)]
pub struct Interpreter {
    registers: HashMap<Register, u64>,
    address: u64,
}

pub fn run(blocks: Vec<Block>) {
    let interpreter = Interpreter::default();
}
