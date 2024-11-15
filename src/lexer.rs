use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
pub enum TokenType {
    Keyword,
    Operator,
    Interupt,
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn lexer(tokens: &Vec<String>) -> HashMap<&str, TokenType> {
    let mut token_stream: HashMap<&str, TokenType> = HashMap::new();
    let keywords: HashMap<&str, TokenType> = HashMap::from([
        ("if", TokenType::Keyword),
        ("else", TokenType::Keyword),
        ("let", TokenType::Keyword),
    ]);
    let operators: HashMap<&str, TokenType> = HashMap::from([
        ("+", TokenType::Keyword),
        ("-", TokenType::Keyword),
        ("/", TokenType::Keyword),
        ("*", TokenType::Keyword),
    ]);
    let interupts: HashMap<&str, TokenType> =
        HashMap::from([("exit", TokenType::Keyword), ("echo", TokenType::Keyword)]);
    for token in tokens {
        if keywords.contains_key(token.as_str()) {
            token_stream.insert(token, TokenType::Keyword);
        } else if operators.contains_key(token.as_str()) {
            token_stream.insert(token, TokenType::Operator);
        } else if interupts.contains_key(token.as_str()) {
            token_stream.insert(token, TokenType::Interupt);
        }
    }
    println!("{:?}", token_stream);
    token_stream
}
