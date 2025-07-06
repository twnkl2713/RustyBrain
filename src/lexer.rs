pub fn tokenize(source: &str) -> Vec<char> {
    source
        .chars()
        .filter(|c| matches!(c, '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']'))
        .collect()
}
