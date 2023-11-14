use std::{fs, any::{self, Any}};

use logos::{Logos, Lexer};

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\f]+")] // Ignore this regex pattern between tokens
enum Token {
    // Tokens can be literal strings, of any length.
    #[token("shake")]
    Shake,

    #[regex("\\{|\\}")]
    CurlyBracket,

    #[regex("\\[|\\]")]
    SquareBracket,

    #[regex("\\(|\\)")]
    Paren,

    #[token(";")]
    SemiColon,

    #[token(":")]
    Colon,

    #[token("=")]
    Equals,

    #[token("//")]
    CommentBegin,

    #[token("\n")]
    Linebreak,

    #[token("\"")]
    DoubleQuote,

    #[regex("[0-9]+")]
    Number,

    #[token("-")]
    Minus,

    #[token("+")]
    Plus,

    #[token("*")]
    Multiply,

    #[token("/")]
    Divide,

    #[token(",")]
    Comma,

    #[token(".")]
    Period,

    #[token("True")]
    True,

    #[token("False")]
    False,

    #[token("bool")]
    TypeBoolean,

    #[token("str")]
    TypeString,

    #[token("num")]
    TypeNumber,

    // Or regular expressions.
    #[regex("[a-zA-Z]+")]
    Text,

    #[token("fc")]
    FunctionDeclaration,

    #[token("const")]
    Constant,

    Program
}

struct Node {
    data: TokenData,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    next: Option<Box<Node>>
}

struct TokenData {
    token: Token,
    slice: String
}

fn get_token(mut lex: Lexer<'_, Token>) -> TokenData {
    let curr = match lex.next() {
        Some(val) => val,
        None => panic!("panicking!")
    };
    let token = match curr {
        Ok(val) => val,
        Err(e) => panic!("{:?}", e)
    };

    return TokenData {
        token,
        slice: lex.slice().to_string()
    };
}

fn parse_equals(token_data: TokenData, lex: Lexer<'_, Token>) -> &Node {
    let node = Node {
        data: token_data,
        left: None,
        right: None,
        next: None
    };

    return &node;
}

fn parse_variable(token_data: TokenData, lex: Lexer<'_, Token>) -> &Node {
    let node = Node {
        data: token_data,
        left: None,
        right: None,
        next: None
    };

    let first_char = token_data.slice.chars().next().unwrap();

    if first_char.is_uppercase() {
        panic!("uppercase is not allowed for a variable");
    }

    let next_token_data = get_token(lex);

    if next_token_data.token != Token::Equals {

        panic!("next char is not equals!");
    }

    let equals_node = parse_equals(next_token_data, lex);

    equals_node.left = Some(Box::new(node));

    return &equals_node;
}

fn program(lex: Lexer<'_, Token>) -> &Node {
    let token_data = get_token(lex);

    let node_token_data = TokenData {
        token: Token::Program,
        slice: "".to_string()
    };

    let node = Node {
        data: node_token_data,
        left: None,
        right: None,
        next: None
    };

    if token_data.token == Token::Text {
        let next_node = parse_variable(token_data, lex);

        node.next = Some(Box::new(next_node));

    } else {
        panic!("unsupported character. only variables supported for now");
    }

    return &node;
}

fn main() {
    let text = match fs::read_to_string("./design.bro") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file {:?}", error)
    };

    let lex = Token::lexer(&text);
    let root = program(lex);
}


