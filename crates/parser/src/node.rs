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
}
