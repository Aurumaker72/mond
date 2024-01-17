use mond::tokenizer::tokenize;

#[test]
fn it_works() {
    let tokens = tokenize("let a = 5 + 2\nlet b = 4 * 2 / 5\nlet c = a + b");
    for token in tokens {
        println!("{:?}", token);
    }
}
