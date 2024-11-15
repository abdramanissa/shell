#[allow(dead_code)]
#[allow(unused_variables)]
pub fn tokenize(buffer: &str) -> Result<Vec<String>, &str> {
    let tokens: Vec<String> = buffer
        .split(' ')
        .map(|token| token.trim().to_string())
        .collect();
    Ok(tokens)
}

#[allow(dead_code)]
pub fn clean(tokens: &mut Vec<String>, buffer: &mut String) {
    tokens.clear();
    buffer.clear();
}
