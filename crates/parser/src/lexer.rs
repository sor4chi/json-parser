use crate::{
    token::{Token, CHAR_TOKENS, KEYWORD_TOKENS},
    utility::PeekableIter,
};

pub struct Lexer {
    char_stream: PeekableIter<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let vec: Vec<char> = input.chars().collect();
        let char_stream = vec.into_iter().peekable();
        Lexer { char_stream }
    }

    fn consume_char(&mut self) -> Token {
        match self.char_stream.next() {
            Some(c) => match CHAR_TOKENS.get(&c) {
                Some(token) => token.clone(),
                None => panic!("Unexpected character: {}", c),
            },
            None => panic!("Unexpected char of input"),
        }
    }

    fn consume_string(&mut self) -> Token {
        if self.char_stream.peek() == Some(&'"') {
            self.char_stream.next(); // the first "
        }
        let mut s = String::new();
        loop {
            match self.char_stream.next() {
                Some('"') => break,
                Some(c) => s.push(c),
                None => panic!("Unexpected end of input"),
            }
        }
        Token::StringValue(s)
    }

    fn consume_number(&mut self) -> Token {
        let mut s = String::new();
        loop {
            match self.char_stream.peek() {
                Some(c) if c.is_numeric() || c == &'.' => match self.char_stream.next() {
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

    fn consume_keyword(&mut self) -> Token {
        let mut keyword = String::new();
        loop {
            let c = self.char_stream.peek();
            match c {
                Some(c) if c.is_alphanumeric() => {
                    keyword.push(*c);
                    self.char_stream.next();
                }
                _ => break,
            }
        }
        match KEYWORD_TOKENS.get(&keyword[..]) {
            Some(token) => token.clone(),
            None => panic!("Unexpected keyword: {}", keyword),
        }
    }

    fn consume_whitespace(&mut self) {
        loop {
            match self.char_stream.peek() {
                Some(c) if c.is_whitespace() => {
                    self.char_stream.next();
                }
                _ => break,
            }
        }
    }

    fn next_token(&mut self) -> Token {
        self.consume_whitespace();
        let c = self.char_stream.peek();
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
    fn test_consume_char() {
        let input = r#"{}[]:,"#;
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.consume_char(), Token::LBrace); // {
        assert_eq!(lexer.consume_char(), Token::RBrace); // }
        assert_eq!(lexer.consume_char(), Token::LBracket); // [
        assert_eq!(lexer.consume_char(), Token::RBracket); // ]
        assert_eq!(lexer.consume_char(), Token::Colon); // :
        assert_eq!(lexer.consume_char(), Token::Comma); // ,
    }

    #[test]
    fn test_consume_string() {
        let input = r#"{"foo":"bar"}"#;
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.consume_char(), Token::LBrace); // {
        assert_eq!(
            lexer.consume_string(),
            Token::StringValue("foo".to_string())
        ); // "foo"
        assert_eq!(lexer.consume_char(), Token::Colon); // :
        assert_eq!(
            lexer.consume_string(),
            Token::StringValue("bar".to_string())
        ); // "bar"
        assert_eq!(lexer.consume_char(), Token::RBrace); // }
    }

    #[test]
    fn test_consume_number() {
        let input = r#"{"foo":123}"#;
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.consume_char(), Token::LBrace); // {
        assert_eq!(
            lexer.consume_string(),
            Token::StringValue("foo".to_string())
        ); // "foo"
        assert_eq!(lexer.consume_char(), Token::Colon); // :
        assert_eq!(lexer.consume_number(), Token::NumberValue(123.0)); // 123
        assert_eq!(lexer.consume_char(), Token::RBrace); // }
    }

    #[test]
    fn test_consume_keyword() {
        let input = r#"{"foo":true,"bar":false,"baz":null}"#;
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.consume_char(), Token::LBrace); // {
        assert_eq!(
            lexer.consume_string(),
            Token::StringValue("foo".to_string())
        ); // "foo"
        assert_eq!(lexer.consume_char(), Token::Colon); // :
        assert_eq!(lexer.consume_keyword(), Token::BooleanValue(true)); // true
        assert_eq!(lexer.consume_char(), Token::Comma); // ,
        assert_eq!(
            lexer.consume_string(),
            Token::StringValue("bar".to_string())
        ); // "bar"
        assert_eq!(lexer.consume_char(), Token::Colon); // :
        assert_eq!(lexer.consume_keyword(), Token::BooleanValue(false)); // false
        assert_eq!(lexer.consume_char(), Token::Comma); // ,
        assert_eq!(
            lexer.consume_string(),
            Token::StringValue("baz".to_string())
        ); // "baz"
        assert_eq!(lexer.consume_char(), Token::Colon); // :
        assert_eq!(lexer.consume_keyword(), Token::NullValue); // null
        assert_eq!(lexer.consume_char(), Token::RBrace); // }
    }

    #[test]
    fn test_consume_whitespace() {
        let input = r#"{    "foo": 123
        }"#;
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.consume_char(), Token::LBrace); // {
        lexer.consume_whitespace(); // tab whitespace
        assert_eq!(
            lexer.consume_string(),
            Token::StringValue("foo".to_string())
        ); // "foo"
        assert_eq!(lexer.consume_char(), Token::Colon); // :
        lexer.consume_whitespace(); // space whitespace
        assert_eq!(lexer.consume_number(), Token::NumberValue(123.0)); // 123
        lexer.consume_whitespace(); // new line whitespace
        assert_eq!(lexer.consume_char(), Token::RBrace); // }
    }

    #[test]
    fn test_next_token() {
        let input = r#"{"foo":123}"#;
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::LBrace); // {
        assert_eq!(lexer.next_token(), Token::StringValue("foo".to_string())); // "foo"
        assert_eq!(lexer.next_token(), Token::Colon); // :
        assert_eq!(lexer.next_token(), Token::NumberValue(123.0)); // 123
        assert_eq!(lexer.next_token(), Token::RBrace); // }
        assert_eq!(lexer.next_token(), Token::End); // end
    }

    #[test]
    fn test_tokenize() {
        let input = r#"{"foo":123}"#;
        let mut lexer = Lexer::new(input);
        let tests: Vec<Token> = vec![
            Token::LBrace,                         // {
            Token::StringValue("foo".to_string()), // "foo"
            Token::Colon,                          // :
            Token::NumberValue(123.0),             // 123
            Token::RBrace,                         // }
            Token::End,                            // end
        ];

        for test in tests {
            assert_eq!(lexer.next_token(), test);
        }
    }
}
