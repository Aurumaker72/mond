use mond::generator::{Instruction, Register};
use mond::interpreter::run;
use mond::tokenizer::tokenize;

#[test]
fn it_works() {
    let tokens = tokenize("let a = 5 + 2\nlet b = 4 * 2 / 5\nlet c = a + b");
    for token in tokens {
        println!("{:?}", token);
    }

    let program = vec![
        Instruction::Init(Register::GPR0, 0),
        Instruction::Init(Register::GPR1, 7),
        Instruction::Init(Register::GPR2, 3),
        Instruction::Add(Register::GPR0, Register::GPR1, Register::GPR2),
    ];
    run(program);
}
