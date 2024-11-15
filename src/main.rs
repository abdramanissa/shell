mod interpreter;
mod lexer;
mod tokenizer;

use std::collections::HashMap;

#[allow(unused_imports)]
use std::{
    io::{self, Write},
    net::ToSocketAddrs,
};

#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(unused_assignments)]
fn main() {
    let mut buffer: String = String::new();
    let mut tokens: Vec<String> = Vec::new();
    let mut token_stream: HashMap<&str, lexer::TokenType> = HashMap::new();

    loop {
        print!("$");
        io::stdout().flush().unwrap(); // Flushing the output buffer
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");
        match tokenizer::tokenize(&buffer) {
            Ok(ready_tokens) => tokens = ready_tokens,
            Err(error) => println!("{:?}", error),
        }
        token_stream = lexer::lexer(&mut tokens);
        tokenizer::clean(&mut tokens, &mut buffer);
    }
}
/*
TODO:
    Implement config file;
    Implement startup code;
*/
