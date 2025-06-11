use std::{fs, env};

use picoc::{lexer, parser::Parser, rep::{ctl::Start, TypeAndVal}};

fn main() {
    println!(
        "
    ⠀⠀⠀⠀⠀⣼⣧⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⣼⣿⣿⣧⠀⠀⠀⠀
    ⠀⠀⠀⠾⠿⠿⠿⠿⠷⠀⠀⠀
    ⠀⠀⣼⣆⠀⠀⠀⠀⣰⣧⠀⠀
    ⠀⣼⣿⣿⣆⠀⠀⣰⣿⣿⣧⠀
    ⠾⠟⠿⠿⠿⠧⠼⠿⠿⠿⠻⠷
    picoc: optimizing C89->RV32I compiler
    "
    );

    let src = env::args()
        .nth(2)
        .expect("picoc-error: no source file given");
    println!("picoc-info: received source: {src}");

    let chars = fs::read(src)
        .expect("picoc-error: file dne`")
        .iter()
        .map(|b| *b as char)
        .collect::<Vec<_>>();
    
    let tokens = lexer::lex(&chars).unwrap();
    let mut parser = Parser::new(Start::new(vec![Box::new(TypeAndVal::Bot)]));
    let graph = parser.parse_prg(&tokens).unwrap();
}
