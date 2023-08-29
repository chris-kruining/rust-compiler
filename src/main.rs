#![feature(generators, generator_trait)]
#![feature(iter_advance_by)]
#![feature(let_chains)]

mod language;
mod lexer;

use lexer::tokenize;
use language::hydrogen;

fn main() {
    println!("====START====");
    
    let input = r#"
    let x = 69;
    exit(x);
    "#;

    for token in tokenize::<hydrogen::Token>(input) {
        match token {
            Err(msg) => {
                println!("{:?}", msg);
            },
            Ok(token) => {
                println!("{}:{} {:?}({:?})", token.line, token.column, token.kind, token.value);
            }
        }
    }

    println!("====DONE====")
}
