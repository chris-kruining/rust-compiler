#![feature(generators, generator_trait)]
#![feature(iter_advance_by)]
#![feature(let_chains)]

mod language;
mod lexer;

use std::str::Chars;
use lexer::tokenize;
use language::hydrogen;

use crate::{language::hydrogen::TokenSpecification, lexer::Token};

fn main() {
    println!("====START====");
    
    let input = r#"
    let x = 69;
    exit(x);
    "#;

    for token in tokenize::<Chars, hydrogen::TokenSpecification>(input.chars()).filter(|t| if let Ok(t) = t { t.kind != hydrogen::TokenSpecification::WhiteSpace } else { true }) {
        match token {
            Err(msg) => {
                println!("{:?}", msg);
            },
            Ok(token) => {
                println!("{}:{} {:?}({:?})", token.line, token.column, token.kind, token.value);
            }
        }
    }

    println!("============");

    let mut tokens = tokenize::<Chars, hydrogen::TokenSpecification>("let x = 69;".chars()).filter_map(|t| match t {
        Err(_) | Ok(Token { kind: hydrogen::TokenSpecification::WhiteSpace, .. }) => None,
        Ok(t) => Some(t),
    });
    let ast = hydrogen::AstNode::parse(&mut tokens);

    println!("{:?}", ast);

    println!("====DONE====")
}
