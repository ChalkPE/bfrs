mod computer;
mod tokenizer;
use std::env::args;

fn main() {
    let mut tape = vec![0];
    let code: String = args().collect();

    let tokens = tokenizer::tokenize(&code[..]);
    let exit = computer::compute(&tokens, &mut tape);

    println!("----------------");
    println!("exit code: {}", exit);
    println!("cell dump: {:?}", &tape);
}
