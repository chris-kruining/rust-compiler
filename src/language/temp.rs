
// grammar! {
//     Statement           -> Expression <SemiColon>
//     Expression          -> Declaration | Block | BinaryExpression | Literal | <Identifier>
//     Declaration         -> Let Identifier(name) Equals Expression(value)
//     Block               -> <BracesOpen> Statement* <BracesClose>
//     BinaryExpression    -> AddExpression | SubtractExpression | MultiplyExpression | DevideExpression
//     AddExpression       -> Expression <Plus> Expression
//     SubtractExpression  -> Expression <Minus> Expression
//     MultiplyExpression  -> Expression <Multiply> Expression
//     DevideExpression    -> Expression <Devide> Expression
//     Literal             -> <BooleanLiteral> | <StringLiteral> | <NumberLiteral>
// }

enum Token {
    // Symbols
    SemiColon,
    Equals,
    BracesOpen,
    BracesClose,

    // Keywords
    Let,

    // Literals
    Identifier,
    NumberLiteral,
    StringLiteral,
    BooleanLiteral,
}

enum Node {
    Statement,
    Expression,
    Declaration,
    Block,
    BinaryExpression,
    AddExpression,
    SubtractExpression,
    MultiplyExpression,
    DevideExpression,
    Literal,
}

enum Pattern<Token, Node> {
    Node(Node),
    Token(Token),
    Nested(Vec<Pattern<Token, Node>>),
    OneOf(Vec<Pattern<Token, Node>>),
    Option(Box<Pattern<Token, Node>>),
    List(Box<Pattern<Token, Node>>)
}

impl Node {
    fn fetch_pattern(self) -> Vec<Pattern<Token, Node>> {    
        type T = Token;
        type N = Node;
        type P = Pattern<T, N>;

        match self {
            Node::Statement => vec![ P::Node(N::Expression), P::Token(T::SemiColon) ],
            Node::Expression => vec![ P::OneOf(vec![P::Node(N::Declaration), P::Node(N::Block), P::Node(N::BinaryExpression), P::Node(N::Declaration), P::Token(T::Identifier) ]) ],
            Node::Declaration => vec![ P::Token(T::Let), P::Token(T::Identifier), P::Token(T::Equals), P::Node(N::Expression) ],
            Node::Block => vec![ P::Token(T::BracesOpen), P::Option(Box::new(P::List(Box::new(P::Node(N::Statement))))), P::Token(T::BracesClose) ],
            // Don't want to implement the rest yet
            _ => vec![],
        }
    }
}