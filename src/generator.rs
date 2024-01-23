use crate::tokenizer::Token;
use crate::tokenizer::Token::{Identifier, Keyword};
use crate::Program;


pub fn generate(token_bag: Vec<Vec<Token>>) -> Program {
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
