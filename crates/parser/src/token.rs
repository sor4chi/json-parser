use phf::phf_map;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Token {
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    Comma,
    StringValue(String),
    NumberValue(f64),
    BooleanValue(bool),
    NullValue,
    End,
}

pub static CHAR_TOKENS: phf::Map<char, Token> = phf_map! {
    '{' => Token::LBrace,
    '}' => Token::RBrace,
    '[' => Token::LBracket,
    ']' => Token::RBracket,
    ':' => Token::Colon,
    ',' => Token::Comma,
};

pub static KEYWORD_TOKENS: phf::Map<&'static str, Token> = phf_map! {
    "true" => Token::BooleanValue(true),
    "false" => Token::BooleanValue(false),
    "null" => Token::NullValue,
};
