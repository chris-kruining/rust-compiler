use std::{iter::Peekable, fmt::Debug};
use itertools::structs::MultiPeek;
use crate::lexer::{Tokenizable, Token};
use strum_macros::EnumIter;
use regex::Regex;

#[derive(Default, EnumIter, Debug, Clone, Copy, PartialEq)]
pub enum TokenSpecification {
    // Symbols
    SemiColon,
    Equals,
    GreaterThan,
    LessThan,
    ParenthesisOpen,
    ParenthesisClose,
    BracesOpen,
    BracesClose,
    BracketsOpen,
    BracketsClose,

    // Keywords
    If,
    Else,
    Not,
    Let,
    Exit,

    // Literals
    Identifier,
    NumberLiteral,
    StringLiteral,
    WhiteSpace,

    #[default]
    Unknown,
}

const SEMICOLON: &str = ";";
const EQUALS: &str = "=";
const GREATER_THAN: &str = ">";
const LESS_THAN: &str = "<";
const PARENTHESIS_OPEN: &str = "(";
const PARENTHESIS_CLOSE: &str = ")";
const BRACES_OPEN: &str = "}";
const BRACES_CLOSE: &str = "{";
const BRACKETS_OPEN: &str = "[";
const BRACKETS_CLOSE: &str = "]";

const IF_KEYWORD: &str = "if";
const ELSE_KEYWORD: &str = "else";
const NOT_KEYWORD: &str = "not";
const LET_KEYWORD: &str = "let";
const EXIT_KEYWORD: &str = "exit";

const IDENTIFIER_PATTERN: &str = r"[_a-zA-Z][_a-zA-Z\d]*";
const NUMBER_PATTERN: &str = r"-?[\d_.,]+";
const WHITE_SPACE_PATTERN: &str = r"\s+";

impl Tokenizable for TokenSpecification {    
    fn claim(&self, buffer: &mut MultiPeek<impl Iterator<Item = char>>) -> Option<String> {
        match self {
            // Handle exact 1 to 1 matchtes
            Self::SemiColon => match_exact(SEMICOLON, buffer),
            Self::Equals => match_exact(EQUALS, buffer),
            Self::GreaterThan => match_exact(GREATER_THAN, buffer),
            Self::LessThan => match_exact(LESS_THAN, buffer),
            Self::ParenthesisOpen => match_exact(PARENTHESIS_OPEN, buffer),
            Self::ParenthesisClose => match_exact(PARENTHESIS_CLOSE, buffer),
            Self::BracesOpen => match_exact(BRACES_OPEN, buffer),
            Self::BracesClose => match_exact(BRACES_CLOSE, buffer),
            Self::BracketsOpen => match_exact(BRACKETS_OPEN, buffer),
            Self::BracketsClose => match_exact(BRACKETS_CLOSE, buffer),

            Self::If => match_keyword(IF_KEYWORD, buffer),
            Self::Else => match_keyword(ELSE_KEYWORD, buffer),
            Self::Not => match_keyword(NOT_KEYWORD, buffer),
            Self::Let => match_keyword(LET_KEYWORD, buffer),
            Self::Exit => match_keyword(EXIT_KEYWORD, buffer),

            // Handle cases of unknown length, e.g. keep on peeking until the next character breaks the pattern
            Self::Identifier => match_pattern(IDENTIFIER_PATTERN, buffer),
            Self::NumberLiteral => match_pattern(NUMBER_PATTERN, buffer),
            Self::StringLiteral => match_string(buffer),
            Self::WhiteSpace => match_pattern(WHITE_SPACE_PATTERN, buffer),
            _ => None,
        }
    }
}

fn match_exact(to_match: &str, buffer: &mut MultiPeek<impl Iterator<Item = char>>) -> Option<String> {
    for char in to_match.chars() {
        if let Some(c) = buffer.peek() && *c != char {
            return None;
        }
    }

    let _ = buffer.advance_by(to_match.len());

    Some(to_match.to_owned()) 
}

fn match_keyword(to_match: &str, buffer: &mut MultiPeek<impl Iterator<Item = char>>) -> Option<String> {
    if let Some(res) = match_exact(to_match, buffer) && let Some(c) = buffer.peek() && !(*c).is_alphabetic() {
        return Some(res);
    }

    None
}

fn match_pattern(pattern: &str, buffer: &mut MultiPeek<impl Iterator<Item = char>>) -> Option<String> {
    let re = Regex::new(format!("^{}$", pattern).as_str()).unwrap();
    let mut result = String::default();
    let mut size = 0;

    while let Some(char) = buffer.peek() {
        result.push(*char);

        if !re.is_match(&result) {
            break;
        }

        size += 1;
    }

    if size == 0
    {
        return None;
    }

    buffer.advance_by(size).expect("Failed to advance buffer, this should in theory never happen since the buffer is checked for all these positions");

    Some(result[0..size].to_owned())
}

fn match_string(buffer: &mut MultiPeek<impl Iterator<Item = char>>) -> Option<String> {
    if let Some(c) = buffer.peek() && *c != '"' {
        return None;
    }

    let mut result = String::from('"');
    let mut escaped = false;

    while let Some(c) = buffer.peek() {
        result.push(*c);

        match *c {
            '\\' => {
                escaped = true;
            },
            '"' => {
                if !escaped {
                    break;
                }
            },
            _ => {
                escaped = false;
            }
        }
    }

    buffer.advance_by(result.len()).expect("Failed to advance buffer, this should in theory never happen since the buffer is checked for all these positions");

    Some(result)
}

enum Pattern<Token, Node> {
    Node(Node),
    Token(Token),
    Nested(Vec<Pattern<Token, Node>>),
}

impl<T: Debug + PartialEq, N> Pattern<T, N> {
    fn is_match(tokens: &mut Peekable<impl Iterator<Item = Token<T>>>, pattern: Vec<Pattern<T, N>>) -> Option<Vec<AstNode>> {
        let length = pattern.len();
        let result = pattern.into_iter().map(|s| s.consume(tokens)).filter_map(|step| step).collect::<Vec<_>>();
    
        if length != result.len() {
            return None;
        }
        
        Some(result)
    }

    fn consume(&self, tokens: &mut impl Iterator<Item = Token<T>>) -> Option<AstNode> {
        match self {
            Pattern::Node(node) => self.node(node, tokens),
            Pattern::Token(kind) => self.token(kind, tokens),
            _ => None,
        }
    }

    fn node(&self, _node: &N, tokens: &mut dyn Iterator<Item = Token<T>>) -> Option<AstNode> {
        let Some(token) = tokens.next() else {
            return None;
        };
        
        Some(AstNode::Expression(Expression::Number(token.value)))
    }
    
    fn token(&self, kind: &T, tokens: &mut dyn Iterator<Item = Token<T>>) -> Option<AstNode> {
        let Some(token) = tokens.next() else {
            return None;
        };
    
        if &token.kind != kind {
            return None;
        }
    
        println!("{}:{} {:?}({:?})", token.line, token.column, token.kind, token.value);
    
        Some(AstNode::Token(token.value))
    }
}

#[derive(Debug)]
pub enum AstNode {
    Program { statements: Vec<Statement> },
    Expression(Expression),
    Token(String),
}

impl AstNode {
    pub fn parse(tokens: &mut impl Iterator<Item = Token<TokenSpecification>>) -> Option<Self> {
        let mut it = tokens.peekable();

        program(&mut it)
    }
}

fn program(tokens: &mut Peekable<impl Iterator<Item = Token<TokenSpecification>>>) -> Option<AstNode> {
    let mut statements = vec![];

    while let Some(_) = tokens.peek() {
        match statement(tokens) {
            Some(statement) => statements.push(statement),
            None => return None,
        }
    }

    Some(AstNode::Program { statements: statements })
}

fn statement(tokens: &mut Peekable<impl Iterator<Item = Token<TokenSpecification>>>) -> Option<Statement> {
    let Some(token) = tokens.peek() else {
        return None;
    };

    match token.kind {
        TokenSpecification::Let => declaration(tokens),
        _ => None,
    }
}

enum G {
    Expression
}

fn declaration(tokens: &mut Peekable<impl Iterator<Item = Token<TokenSpecification>>>) -> Option<Statement> {
    type T = TokenSpecification;
    type P = Pattern<T, G>;

    let pattern = vec![P::Token(T::Let), P::Token(T::Identifier), P::Token(T::Equals), P::Node(G::Expression), P::Token(T::SemiColon)];

    let Some(result) = Pattern::is_match(tokens, pattern) else {
        return None;
    };

    let [ _, nameNode, _, valueNode, _] = result.as_slice() else {
        return None;
    };

    let AstNode::Token(name) = nameNode else {
        return None;
    };

    let AstNode::Expression(value) = valueNode else {
        return None;
    };

    Some(Statement::Decleration{ name: name.to_owned(), value: value.clone() })
}

#[derive(Debug, Clone)]
enum Statement {
    Exit { value: Expression },
    Decleration { name: String, value: Expression },
}

#[derive(Debug, Clone)]
enum Expression {
    Number(String),
    Identifier { name: String },
}

#[derive(Debug)]
struct Exit;
