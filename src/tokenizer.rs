#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    Asterisk,
    Plus,
    Question,
    Dot,
    Pipe,
    Char,
    NChar,
    LeftBrace,
    RightBrace,
    Slash,
    Start,
    End,
}
#[derive(Debug, Clone)]
pub struct Token {
    pub ty: TokenType,
    pub value: String,
}
pub fn tokenize(regex: &str) -> Vec<Token> {
    regex
        .chars()
        .map(|c| Token {
            value: c.to_string(),
            ty: match c {
                '^' => TokenType::Start,
                '$' => TokenType::End,
                '(' => TokenType::LeftParen,
                ')' => TokenType::RightParen,
                '*' => TokenType::Asterisk,
                '+' => TokenType::Plus,
                '?' => TokenType::Question,
                '.' => TokenType::Dot,
                '|' => TokenType::Pipe,
                '[' => TokenType::LeftBrace,
                ']' => TokenType::RightBrace,
                '\\' => TokenType::Slash,
                _ => TokenType::Char,
            },
        })
        .collect()
}
