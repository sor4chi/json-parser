use json_parser::node::{Node, SyntaxKind};

struct FormatOptions {
    spaces: usize,
    use_tabs: bool,
    trailing_commas: bool,
    print_width: usize,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            spaces: 4,
            use_tabs: false,
            trailing_commas: false,
            print_width: 80,
        }
    }
}
struct Formatter {
    source: Node,
    indent: usize,
    options: FormatOptions,
}

impl Formatter {
    fn new(source: Node, _options: Option<FormatOptions>) -> Self {
        Formatter {
            source,
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
            SyntaxKind::StringLiteral(text) => format!("\"{}\"", text),
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
            s.push_str(&self.format(child));
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
            s.push_str(&self.format(child));
        }
        self.down_indent();
        s.push('\n');
        s.push_str(&self.indent_string());
        s.push('}');
        s
    }

    fn format(&mut self, node: &Node) -> String {
        match &node.kind {
            SyntaxKind::ObjectLiteralExpression => self.format_object(node),
            SyntaxKind::ArrayLiteralExpression => self.format_array(node),
            SyntaxKind::StringLiteral(_)
            | SyntaxKind::NumberLiteral(_)
            | SyntaxKind::TrueKeyword
            | SyntaxKind::FalseKeyword
            | SyntaxKind::NullKeyword => self.format_primitive(node),
            SyntaxKind::PropertyAssignment => {
                let mut s = String::new();
                s.push_str(&self.format(&node.children[0]));
                s.push(':');
                s.push(' ');
                s.push_str(&self.format(&node.children[1]));
                s
            }
            _ => unreachable!("format called on non-format node, {:?}", node),
        }
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
            let mut formatter = Formatter::new(Node::new(SyntaxKind::End, vec![]), options);
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
            let formatter = Formatter::new(Node::new(SyntaxKind::End, vec![]), None);
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
                        Node::new(SyntaxKind::StringLiteral("hello".to_string()), vec![]),
                        Node::new(SyntaxKind::NumberLiteral(42.0), vec![]),
                        Node::new(SyntaxKind::TrueKeyword, vec![]),
                        Node::new(SyntaxKind::FalseKeyword, vec![]),
                        Node::new(SyntaxKind::NullKeyword, vec![]),
                    ],
                ),
                "[\n    \"hello\",\n    42,\n    true,\n    false,\n    null\n]".to_string(),
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
            ),
        ];

        for (node, expected) in cases {
            let mut formatter = Formatter::new(Node::new(SyntaxKind::End, vec![]), None);
            println!("{}", formatter.format_array(&node));
            assert_eq!(formatter.format_array(&node), expected);
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
                        Node::new(
                            SyntaxKind::PropertyAssignment,
                            vec![
                                Node::new(SyntaxKind::StringLiteral("foo".to_string()), vec![]),
                                Node::new(SyntaxKind::NumberLiteral(42.0), vec![]),
                            ],
                        ),
                        Node::new(
                            SyntaxKind::PropertyAssignment,
                            vec![
                                Node::new(SyntaxKind::StringLiteral("bar".to_string()), vec![]),
                                Node::new(SyntaxKind::TrueKeyword, vec![]),
                            ],
                        ),
                        Node::new(
                            SyntaxKind::PropertyAssignment,
                            vec![
                                Node::new(SyntaxKind::StringLiteral("baz".to_string()), vec![]),
                                Node::new(SyntaxKind::FalseKeyword, vec![]),
                            ],
                        ),
                        Node::new(
                            SyntaxKind::PropertyAssignment,
                            vec![
                                Node::new(SyntaxKind::StringLiteral("qux".to_string()), vec![]),
                                Node::new(SyntaxKind::NullKeyword, vec![]),
                            ],
                        ),
                    ],
                ),
                "{\n    \"hello\": \"world\",\n    \"foo\": 42,\n    \"bar\": true,\n    \"baz\": false,\n    \"qux\": null\n}".to_string(),
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
            ),
        ];

        for (node, expected) in cases {
            let mut formatter = Formatter::new(Node::new(SyntaxKind::End, vec![]), None);
            println!("{}", formatter.format_object(&node));
            assert_eq!(formatter.format_object(&node), expected);
        }
    }

    #[test]
    fn test_format() {
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
            let mut formatter = Formatter::new(Node::new(SyntaxKind::End, vec![]), None);
            println!("{}", formatter.format(&node));
            assert_eq!(formatter.format(&node), expected);
        }
    }
}
