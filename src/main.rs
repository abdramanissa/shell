#![allow(unused_imports)]
use std::{
    io::{self, Write},
    net::ToSocketAddrs,
};

#[allow(dead_code)]
#[allow(unused_variables)]
fn tokenize(buffer: &str) -> Result<Vec<String>, &str> {
    let tokens: Vec<String> = buffer
        .split(' ')
        .map(|token| token.trim().to_string())
        .collect();
    Ok(tokens)
}

#[allow(dead_code)]
fn clear_tokens_buffer(tokens: &mut Vec<String>) {
    tokens.clear();
}

fn lexer(tokens: &Vec<String>) {
    /*

    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq, Hash)]
    enum TokenType {
        Keyword,
        Identifier,
        Number,
        Operator,
        Punctuator,
    }

    fn tokenize(input: &str) -> Vec<(String, TokenType)> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut current_token_type = TokenType::Identifier;

        let keywords = HashMap::from([
            ("let", TokenType::Keyword),
            ("if", TokenType::Keyword),
            ("else", TokenType::Keyword),
        ]);

        let operators = HashMap::from([
            ("+", TokenType::Operator),
            ("-", TokenType::Operator),
            ("*", TokenType::Operator),
            ("/", TokenType::Operator),
        ]);

        let punctuators = HashMap::from([
            ("(", TokenType::Punctuator),
            (")", TokenType::Punctuator),
            ("{", TokenType::Punctuator),
            ("}", TokenType::Punctuator),
            (",", TokenType::Punctuator),
            (";", TokenType::Punctuator),
        ]);

        for c in input.chars() {
            if c.is_alphabetic() || c == '_' {
                current_token.push(c);
                current_token_type = if keywords.contains_key(&current_token) {
                    TokenType::Keyword
                } else {
                    TokenType::Identifier
                };
            } else if c.is_numeric() {
                current_token.push(c);
                current_token_type = TokenType::Number;
            } else if operators.contains_key(&c.to_string()) {
                if !current_token.is_empty() {
                    tokens.push((current_token.clone(), current_token_type));
                    current_token.clear();
                }
                tokens.push((c.to_string(), operators[&c.to_string()]));
            } else if punctuators.contains_key(&c.to_string()) {
                if !current_token.is_empty() {
                    tokens.push((current_token.clone(), current_token_type));
                    current_token.clear();
                }
                tokens.push((c.to_string(), punctuators[&c.to_string()]));
            } else if c.is_whitespace() {
                if !current_token.is_empty() {
                    tokens.push((current_token.clone(), current_token_type));
                    current_token.clear();
                }
            } else {
                // Handle invalid characters
                // ...
            }
        }

        if !current_token.is_empty() {
            tokens.push((current_token, current_token_type));
        }

        tokens
    }

    fn main() {
        let input = "let x = 10 + 20;";
        let tokens = tokenize(input);
        println!("{:?}", tokens);
    }
    */
}

#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(unused_assignments)]
fn main() {
    let mut buffer: String = String::new();
    let mut tokens: Vec<String> = Vec::new();

    loop {
        print!("$");
        io::stdout().flush().unwrap(); // Flushing the output buffer
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");
        match tokenize(&buffer) {
            Ok(ready_tokens) => tokens = ready_tokens,
            Err(error) => println!("{:?}", error),
        }
        clear_tokens_buffer(&mut tokens);
        println!("{:?}", tokens);
    }
    //match token
    //ok => execute
    //err(exit) => break
}
