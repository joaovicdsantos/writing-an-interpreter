use std::io::stdin;

use crate::{lexer::lexer::Lexer, token::token::EOF};

const PROMPT: &str = ">>";

pub fn start() {
    loop {
        let mut ins = String::new();
        println!("{}", PROMPT);
        stdin().read_line(&mut ins).unwrap();

        let mut l = Lexer::new(ins);
        loop {
            let tok = l.next_token();
            println!("{:?}", tok);
            if tok.r#type == EOF {
                break;
            }
        }
    }
}
