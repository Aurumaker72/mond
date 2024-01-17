use crate::tokenizer::Token::{Identifier, Keyword, Literal, Operator};

// Represents a string with semantics
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Token {
    // A variable's or function's name
    Identifier(String),

    // A mathematical operator
    Operator(String),

    // The literal value of an object
    Literal(String),

    // Language-reserved word, such as let
    Keyword(String),
}

// let a = 5 + 2
// let b = a * 2

// Parses a string into tokens
pub fn tokenize(str: &str) -> Vec<Vec<Token>> {
    let mut tokens = vec![];
    let lines: Vec<_> = str.split("\n").collect();

    for line in lines {
        let mut line_tokens = vec![];
        let parts: Vec<_> = line.split(" ").collect();

        for part in parts {
            if part == "let" {
                line_tokens.push(Keyword(part.to_string()));
            } else if part == "+" || part == "-" || part == "*" || part == "/" || part == "=" {
                line_tokens.push(Operator(part.to_string()));
            } else if part.parse::<i32>().is_ok() {
                line_tokens.push(Literal(part.to_string()));
            } else if part == "" {
            } else {
                line_tokens.push(Identifier(part.to_string()));
            }
        }
        tokens.push(line_tokens);
    }
    tokens
}
