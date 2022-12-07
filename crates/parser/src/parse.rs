use crate::{
    node::{Node, SyntaxKind},
    token::{Token, Tokenizer},
    utility::PeekableIter,
};

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

    fn consume_string(&mut self) -> Node {
        let token = self.next_token();
        match token {
            Some(Token::StringValue(value)) => Node::new(SyntaxKind::StringLiteral(value), vec![]),
            Some(illigal_token) => panic!("Unexpected token: {:?}", illigal_token),
            None => panic!("Unexpected end of input"),
        }
    }

    fn consume_number(&mut self) -> Node {
        let token = self.next_token();
        match token {
            Some(Token::NumberValue(value)) => Node::new(SyntaxKind::NumberLiteral(value), vec![]),
            Some(illegal_token) => panic!("Unexpected token: {:?}", illegal_token),
            None => panic!("Unexpected end of input"),
        }
    }

    fn consume_keyword(&mut self) -> Node {
        let token = self.next_token();
        match token {
            Some(Token::BooleanValue(true)) => Node::new(SyntaxKind::TrueKeyword, vec![]),
            Some(Token::BooleanValue(false)) => Node::new(SyntaxKind::FalseKeyword, vec![]),
            Some(Token::NullValue) => Node::new(SyntaxKind::NullKeyword, vec![]),
            Some(illigal_token) => panic!("Unexpected token: {:?}", illigal_token),
            None => unreachable!("Unexpected token of input"),
        }
    }

    fn consume_property_assignment(&mut self) -> Result<Node, String> {
        let property_name = match self.peek_token() {
            Some(Token::StringValue(s)) => s.clone(),
            _ => return Err("Unexpected token of input".to_string()),
        };
        self.next_token();
        self.next_token();
        match self.consume_value() {
            Ok(value) => Ok(Node::new(
                SyntaxKind::PropertyAssignment,
                vec![
                    Node::new(SyntaxKind::Identifier(property_name), vec![]),
                    value,
                ],
            )),
            Err(e) => Err(e),
        }
    }

    fn consume_object(&mut self) -> Result<Node, String> {
        let mut property_assignments = Vec::new();
        self.next_token();
        loop {
            match self.peek_token() {
                Some(Token::RBrace) => {
                    self.next_token();
                    break;
                }
                Some(Token::StringValue(_)) => match self.consume_property_assignment() {
                    Ok(property_assignment) => {
                        property_assignments.push(property_assignment);
                    }
                    Err(e) => return Err(e),
                },
                Some(Token::Comma) => {
                    self.next_token();
                }
                _ => return Err("Unexpected token of input".to_string()),
            }
        }
        Ok(Node::new(
            SyntaxKind::ObjectLiteralExpression,
            property_assignments,
        ))
    }

    fn consume_array(&mut self) -> Result<Node, String> {
        let mut elements = Vec::new();
        self.next_token();
        loop {
            match self.peek_token() {
                Some(Token::RBracket) => {
                    self.next_token();
                    break;
                }
                Some(Token::StringValue(_))
                | Some(Token::NumberValue(_))
                | Some(Token::BooleanValue(_))
                | Some(Token::NullValue)
                | Some(Token::LBrace)
                | Some(Token::LBracket) => match self.consume_value() {
                    Ok(value) => {
                        elements.push(value);
                    }
                    Err(e) => return Err(e),
                },
                Some(Token::Comma) => {
                    self.next_token();
                }
                _ => return Err("Unexpected token of input".to_string()),
            }
        }
        Ok(Node::new(SyntaxKind::ArrayLiteralExpression, elements))
    }

    fn consume_value(&mut self) -> Result<Node, String> {
        match self.peek_token() {
            Some(Token::StringValue(_)) => Ok(self.consume_string()),
            Some(Token::NumberValue(_)) => Ok(self.consume_number()),
            Some(Token::BooleanValue(_)) | Some(Token::NullValue) => Ok(self.consume_keyword()),
            Some(Token::LBrace) => self.consume_object(),
            Some(Token::LBracket) => self.consume_array(),
            _ => Err("Unexpected token of input".to_string()),
        }
    }

    pub fn parse(&mut self) -> Node {
        let first_token = self.peek_token();
        let result = match first_token {
            Some(Token::LBrace) => self.consume_object(),
            Some(Token::LBracket) => self.consume_array(),
            _ => Err("Unexpected the first token of input".to_string()),
        };
        match result {
            Ok(value) => value,
            Err(e) => panic!("{}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume_string() {
        let mut parser = Parser::new(r#""hello""#);
        assert_eq!(
            parser.next_token(),
            Some(Token::StringValue("hello".to_string()))
        );
    }

    #[test]
    fn test_consume_number() {
        let mut parser = Parser::new("123");
        assert_eq!(parser.next_token(), Some(Token::NumberValue(123.0)));
    }

    #[test]
    fn test_consume_keyword() {
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
    fn test_consume_property_assignment() {
        let success_cases = vec![
            (
                r#""hello": 123"#,
                Ok(Node::new(
                    SyntaxKind::PropertyAssignment,
                    vec![
                        Node::new(SyntaxKind::Identifier("hello".to_string()), vec![]),
                        Node::new(SyntaxKind::NumberLiteral(123.0), vec![]),
                    ],
                )),
            ),
            (
                r#"123: "hello""#,
                Err("Unexpected token of input".to_string()),
            ),
        ];

        for (input, expected) in success_cases {
            let mut parser = Parser::new(input);
            assert_eq!(parser.consume_property_assignment(), expected);
        }
    }

    #[test]
    fn test_consume_object() {
        let cases = vec![
            (
                r#"{"hello": 123}"#,
                Node::new(
                    SyntaxKind::ObjectLiteralExpression,
                    vec![Node::new(
                        SyntaxKind::PropertyAssignment,
                        vec![
                            Node::new(SyntaxKind::Identifier("hello".to_string()), vec![]),
                            Node::new(SyntaxKind::NumberLiteral(123.0), vec![]),
                        ],
                    )],
                ),
            ),
            (
                r#"{"hello": 123, "world": "hello"}"#,
                Node::new(
                    SyntaxKind::ObjectLiteralExpression,
                    vec![
                        Node::new(
                            SyntaxKind::PropertyAssignment,
                            vec![
                                Node::new(SyntaxKind::Identifier("hello".to_string()), vec![]),
                                Node::new(SyntaxKind::NumberLiteral(123.0), vec![]),
                            ],
                        ),
                        Node::new(
                            SyntaxKind::PropertyAssignment,
                            vec![
                                Node::new(SyntaxKind::Identifier("world".to_string()), vec![]),
                                Node::new(SyntaxKind::StringLiteral("hello".to_string()), vec![]),
                            ],
                        ),
                    ],
                ),
            ),
        ];

        for (input, expected) in cases {
            let mut parser = Parser::new(input);
            assert_eq!(parser.parse(), expected);
        }
    }

    #[test]
    fn test_consume_array() {
        let cases = vec![
            (
                r#"[123]"#,
                Node::new(
                    SyntaxKind::ArrayLiteralExpression,
                    vec![Node::new(SyntaxKind::NumberLiteral(123.0), vec![])],
                ),
            ),
            (
                r#"[123, "hello"]"#,
                Node::new(
                    SyntaxKind::ArrayLiteralExpression,
                    vec![
                        Node::new(SyntaxKind::NumberLiteral(123.0), vec![]),
                        Node::new(SyntaxKind::StringLiteral("hello".to_string()), vec![]),
                    ],
                ),
            ),
        ];

        for (input, expected) in cases {
            let mut parser = Parser::new(input);
            assert_eq!(parser.parse(), expected);
        }
    }

    #[test]
    fn test_consume_value() {
        let cases = vec![
            (
                "123",
                Ok(Node::new(SyntaxKind::NumberLiteral(123.0), vec![])),
            ),
            (
                r#""hello""#,
                Ok(Node::new(
                    SyntaxKind::StringLiteral("hello".to_string()),
                    vec![],
                )),
            ),
            ("true", Ok(Node::new(SyntaxKind::TrueKeyword, vec![]))),
            ("false", Ok(Node::new(SyntaxKind::FalseKeyword, vec![]))),
            ("null", Ok(Node::new(SyntaxKind::NullKeyword, vec![]))),
            (
                r#"{"hello": 123}"#,
                Ok(Node::new(
                    SyntaxKind::ObjectLiteralExpression,
                    vec![Node::new(
                        SyntaxKind::PropertyAssignment,
                        vec![
                            Node::new(SyntaxKind::Identifier("hello".to_string()), vec![]),
                            Node::new(SyntaxKind::NumberLiteral(123.0), vec![]),
                        ],
                    )],
                )),
            ),
            (
                r#"[1, 2, 3]"#,
                Ok(Node::new(
                    SyntaxKind::ArrayLiteralExpression,
                    vec![
                        Node::new(SyntaxKind::NumberLiteral(1.0), vec![]),
                        Node::new(SyntaxKind::NumberLiteral(2.0), vec![]),
                        Node::new(SyntaxKind::NumberLiteral(3.0), vec![]),
                    ],
                )),
            ),
            ("", Err("Unexpected token of input".to_string())),
            (
                r#"{"hello": 123"#,
                Err("Unexpected token of input".to_string()),
            ),
        ];

        for (input, expected) in cases {
            let mut parser = Parser::new(input);
            assert_eq!(parser.consume_value(), expected);
        }
    }

    #[test]
    fn test_parse() {
        let cases = vec![
            (
                r#"{"hello": 123}"#,
                Node::new(
                    SyntaxKind::ObjectLiteralExpression,
                    vec![Node::new(
                        SyntaxKind::PropertyAssignment,
                        vec![
                            Node::new(SyntaxKind::Identifier("hello".to_string()), vec![]),
                            Node::new(SyntaxKind::NumberLiteral(123.0), vec![]),
                        ],
                    )],
                ),
            ),
            (
                r#"[1, 2, 3]"#,
                Node::new(
                    SyntaxKind::ArrayLiteralExpression,
                    vec![
                        Node::new(SyntaxKind::NumberLiteral(1.0), vec![]),
                        Node::new(SyntaxKind::NumberLiteral(2.0), vec![]),
                        Node::new(SyntaxKind::NumberLiteral(3.0), vec![]),
                    ],
                ),
            ),
            (
                r#"{"hello": [1, 2, 3]}"#,
                Node::new(
                    SyntaxKind::ObjectLiteralExpression,
                    vec![Node::new(
                        SyntaxKind::PropertyAssignment,
                        vec![
                            Node::new(SyntaxKind::Identifier("hello".to_string()), vec![]),
                            Node::new(
                                SyntaxKind::ArrayLiteralExpression,
                                vec![
                                    Node::new(SyntaxKind::NumberLiteral(1.0), vec![]),
                                    Node::new(SyntaxKind::NumberLiteral(2.0), vec![]),
                                    Node::new(SyntaxKind::NumberLiteral(3.0), vec![]),
                                ],
                            ),
                        ],
                    )],
                ),
            ),
        ];

        for (input, expected) in cases {
            let mut parser = Parser::new(input);
            assert_eq!(parser.parse(), expected);
        }
    }
}
