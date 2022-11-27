#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum SyntaxKind {
    SourceFile,
    StringLiteral(String),
    NumberLiteral(f64),
    TrueKeyword,
    FalseKeyword,
    NullKeyword,
    PropertyAssignment,
    ObjectLiteralExpression,
    ArrayLiteralExpression,
    End,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Node {
    pub kind: SyntaxKind,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(kind: SyntaxKind, children: Vec<Node>) -> Self {
        Node { kind, children }
    }

    pub fn text(&self) -> Option<String> {
        match &self.kind {
            SyntaxKind::StringLiteral(text) => Some(text.clone()),
            SyntaxKind::NumberLiteral(value) => Some(value.to_string()),
            SyntaxKind::TrueKeyword => Some("true".to_string()),
            SyntaxKind::FalseKeyword => Some("false".to_string()),
            SyntaxKind::NullKeyword => Some("null".to_string()),
            _ => None,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text() {
        let cases = vec![
            (
                Node::new(
                    SyntaxKind::StringLiteral("hello".to_string()),
                    vec![],
                ),
                Some("hello".to_string()),
            ),
            (
                Node::new(
                    SyntaxKind::NumberLiteral(42.0),
                    vec![],
                ),
                Some("42".to_string()),
            ),
            (
                Node::new(
                    SyntaxKind::TrueKeyword,
                    vec![],
                ),
                Some("true".to_string()),
            ),
            (
                Node::new(
                    SyntaxKind::FalseKeyword,
                    vec![],
                ),
                Some("false".to_string()),
            ),
            (
                Node::new(
                    SyntaxKind::NullKeyword,
                    vec![],
                ),
                Some("null".to_string()),
            ),
            (
                Node::new(
                    SyntaxKind::PropertyAssignment,
                    vec![],
                ),
                None,
            ),
        ];

        for (node, expected) in cases {
            assert_eq!(node.text(), expected);
        }
    }
}
