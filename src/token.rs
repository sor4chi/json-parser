use phf::phf_map;

use crate::utility::PeekableIter;

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

static CHAR_TOKENS: phf::Map<char, Token> = phf_map! {
    '{' => Token::LBrace,
    '}' => Token::RBrace,
    '[' => Token::LBracket,
    ']' => Token::RBracket,
    ':' => Token::Colon,
    ',' => Token::Comma,
};

static KEYWORD_TOKENS: phf::Map<&'static str, Token> = phf_map! {
    "true" => Token::BooleanValue(true),
    "false" => Token::BooleanValue(false),
    "null" => Token::NullValue,
};

pub struct Tokenizer {
    char_stream: PeekableIter<char>,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        let vec: Vec<char> = input.chars().collect();
        let char_stream = vec.into_iter().peekable();
        Tokenizer { char_stream }
    }

    pub fn peek_char(&mut self) -> Option<&char> {
        self.char_stream.peek()
    }

    pub fn next_char(&mut self) -> Option<char> {
        self.char_stream.next()
    }

    pub fn consume_char(&mut self) -> Token {
        match self.next_char() {
            Some(c) => match CHAR_TOKENS.get(&c) {
                Some(token) => token.clone(),
                None => panic!("Unexpected character: {}", c),
            },
            None => panic!("Unexpected char of input"),
        }
    }

    pub fn consume_string(&mut self) -> Token {
        if self.peek_char() == Some(&'"') {
            self.next_char(); // the first "
        }
        let mut s = String::new();
        loop {
            match self.next_char() {
                Some('"') => break,
                Some(c) => s.push(c),
                None => panic!("Unexpected end of input"),
            }
        }
        Token::StringValue(s)
    }

    pub fn consume_number(&mut self) -> Token {
        let mut s = String::new();
        loop {
            match self.peek_char() {
                Some(c) if c.is_numeric() || c == &'.' => match self.next_char() {
                    Some(c) => s.push(c),
                    None => panic!("Unexpected end of input"),
                },
                _ => break,
            }
        }
        match s.parse::<f64>() {
            Ok(n) => Token::NumberValue(n),
            Err(_) => panic!("Unexpected number: {}", s),
        }
    }

    pub fn consume_keyword(&mut self) -> Token {
        let mut keyword = String::new();
        loop {
            let c = self.peek_char();
            match c {
                Some(c) if c.is_alphanumeric() => {
                    keyword.push(*c);
                    self.next_char();
                }
                _ => break,
            }
        }
        match KEYWORD_TOKENS.get(&keyword[..]) {
            Some(token) => token.clone(),
            None => panic!("Unexpected keyword: {}", keyword),
        }
    }

    pub fn consume_whitespace(&mut self) {
        loop {
            match self.peek_char() {
                Some(c) if c.is_whitespace() => {
                    self.next_char();
                }
                _ => break,
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.consume_whitespace();
        let c = self.peek_char();
        match c {
            Some(c) => match c {
                '{' | '}' | '[' | ']' | ':' | ',' => self.consume_char(),
                '"' => self.consume_string(),
                '0'..='9' => self.consume_number(),
                'a'..='z' | 'A'..='Z' => self.consume_keyword(),
                _ => panic!("Unexpected character: {}", c),
            },
            None => Token::End,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            tokens.push(token.clone());
            if token == Token::End {
                break;
            }
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peek_char() {
        let input = r#"{"foo":123}"#;
        let mut tokenizer = Tokenizer::new(input);
        assert_eq!(tokenizer.peek_char(), Some(&'{'));
        assert_eq!(tokenizer.peek_char(), Some(&'{'));
    }

    #[test]
    fn test_next_char() {
        let input = r#"{"foo":123}"#;
        let mut tokenizer = Tokenizer::new(input);
        assert_eq!(tokenizer.next_char(), Some('{'));
        assert_eq!(tokenizer.next_char(), Some('"'));
        assert_eq!(tokenizer.next_char(), Some('f'));
        assert_eq!(tokenizer.next_char(), Some('o'));
        assert_eq!(tokenizer.next_char(), Some('o'));
        assert_eq!(tokenizer.next_char(), Some('"'));
        assert_eq!(tokenizer.next_char(), Some(':'));
        assert_eq!(tokenizer.next_char(), Some('1'));
        assert_eq!(tokenizer.next_char(), Some('2'));
        assert_eq!(tokenizer.next_char(), Some('3'));
        assert_eq!(tokenizer.next_char(), Some('}'));
        assert_eq!(tokenizer.next_char(), None);
    }

    #[test]
    fn test_consume_char() {
        let input = r#"{}[]:,"#;
        let mut tokenizer = Tokenizer::new(input);
        assert_eq!(tokenizer.consume_char(), Token::LBrace); // {
        assert_eq!(tokenizer.consume_char(), Token::RBrace); // }
        assert_eq!(tokenizer.consume_char(), Token::LBracket); // [
        assert_eq!(tokenizer.consume_char(), Token::RBracket); // ]
        assert_eq!(tokenizer.consume_char(), Token::Colon); // :
        assert_eq!(tokenizer.consume_char(), Token::Comma); // ,
    }

    #[test]
    fn test_consume_string() {
        let input = r#"{"foo":"bar"}"#;
        let mut tokenizer = Tokenizer::new(input);
        assert_eq!(tokenizer.consume_char(), Token::LBrace); // {
        assert_eq!(tokenizer.consume_string(), Token::StringValue("foo".to_string())); // "foo"
        assert_eq!(tokenizer.consume_char(), Token::Colon); // :
        assert_eq!(tokenizer.consume_string(), Token::StringValue("bar".to_string())); // "bar"
        assert_eq!(tokenizer.consume_char(), Token::RBrace); // }
    }

    #[test]
    fn test_consume_number() {
        let input = r#"{"foo":123}"#;
        let mut tokenizer = Tokenizer::new(input);
        assert_eq!(tokenizer.consume_char(), Token::LBrace); // {
        assert_eq!(tokenizer.consume_string(), Token::StringValue("foo".to_string())); // "foo"
        assert_eq!(tokenizer.consume_char(), Token::Colon); // :
        assert_eq!(tokenizer.consume_number(), Token::NumberValue(123.0)); // 123
        assert_eq!(tokenizer.consume_char(), Token::RBrace); // }
    }

    #[test]
    fn test_consume_keyword() {
        let input = r#"{"foo":true,"bar":false,"baz":null}"#;
        let mut tokenizer = Tokenizer::new(input);
        assert_eq!(tokenizer.consume_char(), Token::LBrace); // {
        assert_eq!(tokenizer.consume_string(), Token::StringValue("foo".to_string())); // "foo"
        assert_eq!(tokenizer.consume_char(), Token::Colon); // :
        assert_eq!(tokenizer.consume_keyword(), Token::BooleanValue(true)); // true
        assert_eq!(tokenizer.consume_char(), Token::Comma); // ,
        assert_eq!(tokenizer.consume_string(), Token::StringValue("bar".to_string())); // "bar"
        assert_eq!(tokenizer.consume_char(), Token::Colon); // :
        assert_eq!(tokenizer.consume_keyword(), Token::BooleanValue(false)); // false
        assert_eq!(tokenizer.consume_char(), Token::Comma); // ,
        assert_eq!(tokenizer.consume_string(), Token::StringValue("baz".to_string())); // "baz"
        assert_eq!(tokenizer.consume_char(), Token::Colon); // :
        assert_eq!(tokenizer.consume_keyword(), Token::NullValue); // null
        assert_eq!(tokenizer.consume_char(), Token::RBrace); // }
    }

    #[test]
    fn test_consume_whitespace() {
        let input = r#"{    "foo": 123
        }"#;
        let mut tokenizer = Tokenizer::new(input);
        assert_eq!(tokenizer.consume_char(), Token::LBrace); // {
        tokenizer.consume_whitespace(); // tab whitespace
        assert_eq!(tokenizer.consume_string(), Token::StringValue("foo".to_string())); // "foo"
        assert_eq!(tokenizer.consume_char(), Token::Colon); // :
        tokenizer.consume_whitespace(); // space whitespace
        assert_eq!(tokenizer.consume_number(), Token::NumberValue(123.0)); // 123
        tokenizer.consume_whitespace(); // new line whitespace
        assert_eq!(tokenizer.consume_char(), Token::RBrace); // }
    }

    #[test]
    fn test_next_token() {
        let input = r#"{"foo":123}"#;
        let mut tokenizer = Tokenizer::new(input);
        assert_eq!(tokenizer.next_token(), Token::LBrace); // {
        assert_eq!(tokenizer.next_token(), Token::StringValue("foo".to_string())); // "foo"
        assert_eq!(tokenizer.next_token(), Token::Colon); // :
        assert_eq!(tokenizer.next_token(), Token::NumberValue(123.0)); // 123
        assert_eq!(tokenizer.next_token(), Token::RBrace); // }
        assert_eq!(tokenizer.next_token(), Token::End); // end
    }

    #[test]
    fn test_tokenize() {
        let input = r#"{"foo":123}"#;
        let mut tokenizer = Tokenizer::new(input);
        let tests: Vec<Token> = vec![
            Token::LBrace,                    // {
            Token::StringValue("foo".to_string()), // "foo"
            Token::Colon,                     // :
            Token::NumberValue(123.0),             // 123
            Token::RBrace,                    // }
            Token::End,                       // end
        ];

        for test in tests {
            assert_eq!(tokenizer.next_token(), test);
        }
    }
}
