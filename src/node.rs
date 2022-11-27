use crate::{
    token::{Token, Tokenizer},
    utility::PeekableIter,
};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum SyntaxKind {
    SourceFile,
    StringLiteral(String),
    NumberLiteral(f64),
    TrueKeyword,
    FalseKeyword,
    NullKeyword,
    PropertyAssignment(String, Box<SyntaxKind>),
    ObjectLiteralExpression(Vec<SyntaxKind>),
    ArrayLiteralExpression(Vec<SyntaxKind>),
    End,
}

pub struct Parser {
    token_stream: PeekableIter<Token>,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();
        let token_stream = tokens.into_iter().peekable();
        Parser { token_stream }
    }

    pub fn peek_token(&mut self) -> Option<&Token> {
        self.token_stream.peek()
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.token_stream.next()
    }

    pub fn consume_token(&mut self) -> Token {
        match self.next_token() {
            Some(token) => token,
            None => panic!("Unexpected token of input"),
        }
    }

    pub fn consume_string(&mut self) -> SyntaxKind {
        let token = self.consume_token();
        match token {
            Token::StringValue(value) => SyntaxKind::StringLiteral(value),
            _ => panic!("Unexpected token of input"),
        }
    }

    pub fn consume_number(&mut self) -> SyntaxKind {
        let token = self.consume_token();
        match token {
            Token::NumberValue(value) => SyntaxKind::NumberLiteral(value),
            _ => panic!("Unexpected token of input"),
        }
    }

    pub fn consume_keyword(&mut self) -> SyntaxKind {
        let token = self.consume_token();
        match token {
            Token::BooleanValue(true) => SyntaxKind::TrueKeyword,
            Token::BooleanValue(false) => SyntaxKind::FalseKeyword,
            Token::NullValue => SyntaxKind::NullKeyword,
            _ => panic!("Unexpected token of input"),
        }
    }

    pub fn consume_property_assignment(&mut self) -> SyntaxKind {
        let property_name = match self.peek_token() {
            Some(Token::StringValue(s)) => s.clone(),
            _ => panic!("Unexpected token of input"),
        };
        self.consume_token();
        self.consume_token();
        let value = self.parse_value();
        SyntaxKind::PropertyAssignment(property_name, Box::new(value))
    }

    pub fn consume_object(&mut self) -> SyntaxKind {
        let mut property_assignments = Vec::new();
        self.consume_token();
        loop {
            match self.peek_token() {
                Some(Token::RBrace) => {
                    self.consume_token();
                    break;
                }
                Some(Token::StringValue(_)) => {
                    let property_assignment = self.consume_property_assignment();
                    property_assignments.push(property_assignment);
                }
                Some(Token::Comma) => {
                    self.consume_token();
                }
                _ => panic!("Unexpected token of input"),
            }
        }
        SyntaxKind::ObjectLiteralExpression(property_assignments)
    }

    pub fn consume_array(&mut self) -> SyntaxKind {
        let mut elements = Vec::new();
        self.consume_token();
        loop {
            match self.peek_token() {
                Some(Token::RBracket) => {
                    self.consume_token();
                    break;
                }
                Some(Token::StringValue(_))
                | Some(Token::NumberValue(_))
                | Some(Token::BooleanValue(_))
                | Some(Token::NullValue)
                | Some(Token::LBrace)
                | Some(Token::LBracket) => {
                    let value = self.parse_value();
                    elements.push(value);
                }
                Some(Token::Comma) => {
                    self.consume_token();
                }
                _ => panic!("Unexpected token of input"),
            }
        }
        SyntaxKind::ArrayLiteralExpression(elements)
    }

    pub fn parse_value(&mut self) -> SyntaxKind {
        match self.peek_token() {
            Some(Token::StringValue(_)) => self.consume_string(),
            Some(Token::NumberValue(_)) => self.consume_number(),
            Some(Token::BooleanValue(_)) | Some(Token::NullValue) => self.consume_keyword(),
            Some(Token::LBrace) => self.consume_object(),
            Some(Token::LBracket) => self.consume_array(),
            _ => panic!("Unexpected token of input"),
        }
    }

    pub fn parse(&mut self) -> SyntaxKind {
        let first_token = self.peek_token();
        match first_token {
            Some(Token::LBrace) => self.consume_object(),
            Some(Token::LBracket) => self.consume_array(),
            _ => panic!("Unexpected the first token of input"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        let mut parser = Parser::new(r#""hello""#);
        assert_eq!(
            parser.next_token(),
            Some(Token::StringValue("hello".to_string()))
        );
    }

    #[test]
    fn test_parse_number() {
        let mut parser = Parser::new("123");
        assert_eq!(parser.next_token(), Some(Token::NumberValue(123.0)));
    }

    #[test]
    fn test_parse_keyword() {
        let cases = vec![
            ("true", Token::BooleanValue(true)),
            ("false", Token::BooleanValue(false)),
            ("null", Token::NullValue),
        ];

        for (input, expected) in cases {
            let mut parser = Parser::new(input);
            assert_eq!(parser.next_token(), Some(expected));
        }
    }

    #[test]
    fn test_parse_property_assignment() {
        let mut parser = Parser::new(r#""hello": 123"#);
        assert_eq!(
            parser.next_token(),
            Some(Token::StringValue("hello".to_string()))
        );
        assert_eq!(parser.next_token(), Some(Token::Colon));
        assert_eq!(parser.next_token(), Some(Token::NumberValue(123.0)));
    }

    #[test]
    fn test_parse_object() {
        let mut parser = Parser::new(r#"{"hello": 123}"#);
        assert_eq!(parser.next_token(), Some(Token::LBrace));
        assert_eq!(
            parser.next_token(),
            Some(Token::StringValue("hello".to_string()))
        );
        assert_eq!(parser.next_token(), Some(Token::Colon));
        assert_eq!(parser.next_token(), Some(Token::NumberValue(123.0)));
        assert_eq!(parser.next_token(), Some(Token::RBrace));
    }

    #[test]
    fn test_parse_array() {
        let mut parser = Parser::new(r#"[1, 2, 3]"#);
        assert_eq!(parser.next_token(), Some(Token::LBracket));
        assert_eq!(parser.next_token(), Some(Token::NumberValue(1.0)));
        assert_eq!(parser.next_token(), Some(Token::Comma));
        assert_eq!(parser.next_token(), Some(Token::NumberValue(2.0)));
        assert_eq!(parser.next_token(), Some(Token::Comma));
        assert_eq!(parser.next_token(), Some(Token::NumberValue(3.0)));
        assert_eq!(parser.next_token(), Some(Token::RBracket));
    }

    #[test]
    fn test_parse_value() {
        let cases = vec![
            ("123", SyntaxKind::NumberLiteral(123.0)),
            (r#""hello""#, SyntaxKind::StringLiteral("hello".to_string())),
            ("true", SyntaxKind::TrueKeyword),
            ("false", SyntaxKind::FalseKeyword),
            ("null", SyntaxKind::NullKeyword),
            (
                r#"{"hello": 123}"#,
                SyntaxKind::ObjectLiteralExpression(vec![SyntaxKind::PropertyAssignment(
                    "hello".to_string(),
                    Box::new(SyntaxKind::NumberLiteral(123.0)),
                )]),
            ),
            (
                r#"[1, 2, 3]"#,
                SyntaxKind::ArrayLiteralExpression(vec![
                    SyntaxKind::NumberLiteral(1.0),
                    SyntaxKind::NumberLiteral(2.0),
                    SyntaxKind::NumberLiteral(3.0),
                ]),
            ),
        ];

        for (input, expected) in cases {
            let mut parser = Parser::new(input);
            assert_eq!(parser.parse_value(), expected);
        }
    }

    #[test]
    fn test_parse() {
        let cases = vec![
            (
                r#"{"hello": 123}"#,
                SyntaxKind::ObjectLiteralExpression(vec![SyntaxKind::PropertyAssignment(
                    "hello".to_string(),
                    Box::new(SyntaxKind::NumberLiteral(123.0)),
                )]),
            ),
            (
                r#"[1, 2, 3]"#,
                SyntaxKind::ArrayLiteralExpression(vec![
                    SyntaxKind::NumberLiteral(1.0),
                    SyntaxKind::NumberLiteral(2.0),
                    SyntaxKind::NumberLiteral(3.0),
                ]),
            ),
            (
                r#"{"hello": [1, 2, 3]}"#,
                SyntaxKind::ObjectLiteralExpression(vec![SyntaxKind::PropertyAssignment(
                    "hello".to_string(),
                    Box::new(SyntaxKind::ArrayLiteralExpression(vec![
                        SyntaxKind::NumberLiteral(1.0),
                        SyntaxKind::NumberLiteral(2.0),
                        SyntaxKind::NumberLiteral(3.0),
                    ])),
                )]),
            ),
        ];

        for (input, expected) in cases {
            let mut parser = Parser::new(input);
            assert_eq!(parser.parse(), expected);
        }
    }
}
