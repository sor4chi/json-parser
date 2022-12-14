use json_parser::{
    node::{Node, SyntaxKind},
    parse::Parser,
};

pub struct FormatOptions {
    pub spaces: usize,
    pub use_tabs: bool,
    pub trailing_commas: bool,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            spaces: 4,
            use_tabs: false,
            trailing_commas: false,
        }
    }
}

pub struct Formatter {
    indent: usize,
    options: FormatOptions,
}

impl Formatter {
    pub fn new(_options: Option<FormatOptions>) -> Self {
        Formatter {
            indent: 0,
            options: _options.unwrap_or_default(),
        }
    }

    fn up_indent(&mut self) {
        self.indent += 1;
    }

    fn down_indent(&mut self) {
        self.indent -= 1;
    }

    fn indent_string(&self) -> String {
        let mut s = String::new();
        for _ in 0..self.indent {
            if self.options.use_tabs {
                s.push('\t');
            } else {
                for _ in 0..self.options.spaces {
                    s.push(' ');
                }
            }
        }
        s
    }

    fn format_primitive(&self, node: &Node) -> String {
        match &node.kind {
            SyntaxKind::StringLiteral(text) | SyntaxKind::Identifier(text) => {
                format!("\"{}\"", text)
            }
            SyntaxKind::NumberLiteral(value) => value.to_string(),
            SyntaxKind::TrueKeyword => "true".to_string(),
            SyntaxKind::FalseKeyword => "false".to_string(),
            SyntaxKind::NullKeyword => "null".to_string(),
            _ => unreachable!("format_primitive called on non-primitive node, {:?}", node),
        }
    }

    fn format_array(&mut self, node: &Node) -> String {
        let mut s = String::new();
        s.push('[');
        self.up_indent();
        let mut first = true;
        for child in &node.children {
            if first {
                first = false;
            } else {
                s.push(',');
            }
            s.push('\n');
            s.push_str(&self.indent_string());
            s.push_str(&self.format_node(child));
        }
        if self.options.trailing_commas {
            s.push(',');
        }
        self.down_indent();
        s.push('\n');
        s.push_str(&self.indent_string());
        s.push(']');
        s
    }

    fn format_object(&mut self, node: &Node) -> String {
        let mut s = String::new();
        s.push('{');
        self.up_indent();
        let mut first = true;
        for child in &node.children {
            if first {
                first = false;
            } else {
                s.push(',');
            }
            s.push('\n');
            s.push_str(&self.indent_string());
            s.push_str(&self.format_node(child));
        }
        if self.options.trailing_commas {
            s.push(',');
        }
        self.down_indent();
        s.push('\n');
        s.push_str(&self.indent_string());
        s.push('}');
        s
    }

    fn format_node(&mut self, node: &Node) -> String {
        match &node.kind {
            SyntaxKind::ObjectLiteralExpression => self.format_object(node),
            SyntaxKind::ArrayLiteralExpression => self.format_array(node),
            SyntaxKind::StringLiteral(_)
            | SyntaxKind::NumberLiteral(_)
            | SyntaxKind::Identifier(_)
            | SyntaxKind::TrueKeyword
            | SyntaxKind::FalseKeyword
            | SyntaxKind::NullKeyword => self.format_primitive(node),
            SyntaxKind::PropertyAssignment => {
                let mut s = String::new();
                s.push_str(&self.format_node(&node.children[0]));
                s.push(':');
                s.push(' ');
                s.push_str(&self.format_node(&node.children[1]));
                s
            }
            _ => unreachable!("format called on non-format node, {:?}", node),
        }
    }

    pub fn format(&mut self, input: &str) -> String {
        let mut parser = Parser::new(input);
        let node = parser.parse();
        self.format_node(&node)
    }
}

#[cfg(test)]
mod tests {
    use json_parser::node::SyntaxKind;

    use super::*;

    #[test]
    fn test_indent_string() {
        let cases = vec![
            (None, "    "),
            (
                Some(FormatOptions {
                    spaces: 2,
                    ..Default::default()
                }),
                "  ",
            ),
            (
                Some(FormatOptions {
                    use_tabs: true,
                    ..Default::default()
                }),
                "\t",
            ),
            (
                Some(FormatOptions {
                    spaces: 2,
                    use_tabs: true,
                    ..Default::default()
                }),
                "\t",
            ),
        ];

        for (options, expected) in cases {
            let mut formatter = Formatter::new(options);
            formatter.up_indent();
            assert_eq!(formatter.indent_string(), expected);
        }
    }

    #[test]
    fn test_format_primitive() {
        let cases = vec![
            (
                Node::new(SyntaxKind::StringLiteral("hello".to_string()), vec![]),
                "\"hello\"".to_string(),
            ),
            (
                Node::new(SyntaxKind::NumberLiteral(42.0), vec![]),
                "42".to_string(),
            ),
            (
                Node::new(SyntaxKind::TrueKeyword, vec![]),
                "true".to_string(),
            ),
            (
                Node::new(SyntaxKind::FalseKeyword, vec![]),
                "false".to_string(),
            ),
            (
                Node::new(SyntaxKind::NullKeyword, vec![]),
                "null".to_string(),
            ),
        ];

        for (node, expected) in cases {
            let formatter = Formatter::new(None);
            assert_eq!(formatter.format_primitive(&node), expected);
        }
    }

    #[test]
    fn test_format_array() {
        let cases = vec![
            (
                Node::new(
                    SyntaxKind::ArrayLiteralExpression,
                    vec![
                        Node::new(SyntaxKind::NumberLiteral(1.0), vec![]),
                        Node::new(SyntaxKind::NumberLiteral(2.0), vec![]),
                    ],
                ),
                "[\n    1,\n    2\n]".to_string(),
                "[\n    1,\n    2,\n]".to_string(),
            ),
            (
                Node::new(
                    SyntaxKind::ArrayLiteralExpression,
                    vec![
                        Node::new(
                            SyntaxKind::ArrayLiteralExpression,
                            vec![Node::new(SyntaxKind::NumberLiteral(1.0), vec![])],
                        ),
                        Node::new(
                            SyntaxKind::ArrayLiteralExpression,
                            vec![Node::new(SyntaxKind::NumberLiteral(2.0), vec![])],
                        ),
                    ],
                ),
                "[\n    [\n        1\n    ],\n    [\n        2\n    ]\n]".to_string(),
                "[\n    [\n        1,\n    ],\n    [\n        2,\n    ],\n]".to_string(),
            ),
        ];

        for (node, expected, expected_with_trailing_commas) in cases {
            let mut formatter = Formatter::new(None);
            assert_eq!(formatter.format_array(&node), expected);
            formatter.options.trailing_commas = true;
            assert_eq!(formatter.format_array(&node), expected_with_trailing_commas);
        }
    }

    #[test]
    fn test_format_object() {
        let cases = vec![
            (
                Node::new(
                    SyntaxKind::ObjectLiteralExpression,
                    vec![
                        Node::new(
                            SyntaxKind::PropertyAssignment,
                            vec![
                                Node::new(SyntaxKind::StringLiteral("hello".to_string()), vec![]),
                                Node::new(SyntaxKind::StringLiteral("world".to_string()), vec![]),
                            ],
                        ),
                    ],
                ),
                "{\n    \"hello\": \"world\"\n}".to_string(),
                "{\n    \"hello\": \"world\",\n}".to_string(),
            ),
            (
                Node::new(
                    SyntaxKind::ObjectLiteralExpression,
                    vec![
                        Node::new(
                            SyntaxKind::PropertyAssignment,
                            vec![
                                Node::new(SyntaxKind::StringLiteral("hello".to_string()), vec![]),
                                Node::new(
                                    SyntaxKind::ObjectLiteralExpression,
                                    vec![
                                        Node::new(
                                            SyntaxKind::PropertyAssignment,
                                            vec![
                                                Node::new(SyntaxKind::StringLiteral("foo".to_string()), vec![]),
                                                Node::new(SyntaxKind::NumberLiteral(42.0), vec![]),
                                            ],
                                        ),
                                    ],
                                ),
                            ],
                        ),
                        Node::new(
                            SyntaxKind::PropertyAssignment,
                            vec![
                                Node::new(SyntaxKind::StringLiteral("world".to_string()), vec![]),
                                Node::new(
                                    SyntaxKind::ObjectLiteralExpression,
                                    vec![
                                        Node::new(
                                            SyntaxKind::PropertyAssignment,
                                            vec![
                                                Node::new(SyntaxKind::StringLiteral("bar".to_string()), vec![]),
                                                Node::new(SyntaxKind::NumberLiteral(42.0), vec![]),
                                            ],
                                        ),
                                    ],
                                ),
                            ],
                        ),
                    ],
                ),
                "{\n    \"hello\": {\n        \"foo\": 42\n    },\n    \"world\": {\n        \"bar\": 42\n    }\n}".to_string(),
                "{\n    \"hello\": {\n        \"foo\": 42,\n    },\n    \"world\": {\n        \"bar\": 42,\n    },\n}".to_string(),
            ),
        ];

        for (node, expected, expected_with_trailing_commas) in cases {
            let mut formatter = Formatter::new(None);
            assert_eq!(formatter.format_object(&node), expected);
            formatter.options.trailing_commas = true;
            assert_eq!(
                formatter.format_object(&node),
                expected_with_trailing_commas
            );
        }
    }

    #[test]
    fn test_format_node() {
        let cases = vec![
            (
                Node::new(
                    SyntaxKind::ArrayLiteralExpression,
                    vec![
                        Node::new(SyntaxKind::StringLiteral("hello".to_string()), vec![]),
                        Node::new(SyntaxKind::NumberLiteral(42.0), vec![]),
                    ],
                ),
                "[\n    \"hello\",\n    42\n]".to_string(),
            ),
            (
                Node::new(
                    SyntaxKind::ObjectLiteralExpression,
                    vec![
                        Node::new(
                            SyntaxKind::PropertyAssignment,
                            vec![
                                Node::new(SyntaxKind::StringLiteral("hello".to_string()), vec![]),
                                Node::new(SyntaxKind::ObjectLiteralExpression, vec![
                                    Node::new(
                                        SyntaxKind::PropertyAssignment,
                                        vec![
                                            Node::new(SyntaxKind::StringLiteral("foo".to_string()), vec![]),
                                            Node::new(SyntaxKind::NumberLiteral(42.0), vec![]),
                                        ],
                                    ),
                                ]),
                            ],
                        ),
                        Node::new(
                            SyntaxKind::PropertyAssignment,
                            vec![
                                Node::new(SyntaxKind::StringLiteral("world".to_string()), vec![]),
                                Node::new(SyntaxKind::ArrayLiteralExpression, vec![
                                    Node::new(SyntaxKind::StringLiteral("bar".to_string()), vec![]),
                                    Node::new(SyntaxKind::NumberLiteral(42.0), vec![]),
                                ]),
                            ],
                        ),
                    ],
                ),
                "{\n    \"hello\": {\n        \"foo\": 42\n    },\n    \"world\": [\n        \"bar\",\n        42\n    ]\n}".to_string(),
            ),
        ];

        for (node, expected) in cases {
            let mut formatter = Formatter::new(None);
            assert_eq!(formatter.format_node(&node), expected);
        }
    }

    #[test]
    fn test_format() {
        let cases = vec![
            (
                r#"{"hello": "world"}"#,
                "{\n    \"hello\": \"world\"\n}".to_string(),
            ),
            (
                r#"[1, 2, 3]"#,
                "[\n    1,\n    2,\n    3\n]".to_string(),
            ),
            (
                r#"{"hello": {"foo": 42}, "world": ["bar", 42]}"#,
                "{\n    \"hello\": {\n        \"foo\": 42\n    },\n    \"world\": [\n        \"bar\",\n        42\n    ]\n}".to_string(),
            ),
            (
                r#"[{"hello": "world"}, {"foo": "bar"}]"#,
                "[\n    {\n        \"hello\": \"world\"\n    },\n    {\n        \"foo\": \"bar\"\n    }\n]".to_string(),
            )
        ];

        for (input, expected) in cases {
            let mut formatter = Formatter::new(None);
            assert_eq!(formatter.format(&input), expected);
        }
    }
}
