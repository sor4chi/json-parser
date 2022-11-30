#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum SyntaxKind {
    SourceFile,
    StringLiteral(String),
    NumberLiteral(f64),
    TrueKeyword,
    FalseKeyword,
    NullKeyword,
    PropertyAssignment,
    Identifier(String),
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
            SyntaxKind::Identifier(text) => Some(text.clone()),
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
                SyntaxKind::StringLiteral("foo".to_string()),
                Some("foo".to_string()),
            ),
            (SyntaxKind::NumberLiteral(1.0), Some("1".to_string())),
            (
                SyntaxKind::Identifier("foo".to_string()),
                Some("foo".to_string()),
            ),
            (SyntaxKind::TrueKeyword, Some("true".to_string())),
            (SyntaxKind::FalseKeyword, Some("false".to_string())),
            (SyntaxKind::NullKeyword, Some("null".to_string())),
            (SyntaxKind::SourceFile, None),
            (SyntaxKind::PropertyAssignment, None),
            (SyntaxKind::ObjectLiteralExpression, None),
            (SyntaxKind::ArrayLiteralExpression, None),
            (SyntaxKind::End, None),
        ];

        for (kind, expected) in cases {
            let node = Node::new(kind, vec![]);
            assert_eq!(node.text(), expected);
        }
    }
}
