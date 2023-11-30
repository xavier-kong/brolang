use std::fs;

mod action;

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

#[derive(Debug)]
pub struct Node {
    data: TokenData,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    next: Option<Box<Node>>
}

#[derive(Debug)]
struct TokenData {
    token: Token,
    slice: String
}

fn get_token(lex: &mut Lexer<'_, Token>) -> TokenData {
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

fn create_new_node(token_data: TokenData) -> Node {
    let node = Node {
        data: token_data,
        left: None,
        right: None,
        next: None
    };

    return node;
}

fn parse_minus(token_data: TokenData, lex: & mut Lexer<'_, Token>) -> Node {
    let mut node = create_new_node(token_data);

    let next_token_data = get_token(lex);

    if next_token_data.token == Token::Number {
        let number_node = parse_number(next_token_data, lex);
        node.right = Some(Box::new(number_node));
    } else {
        panic!("current invalid input for after minus");
    }

    return node;
}

fn parse_plus(token_data: TokenData, lex: & mut Lexer<'_, Token>) -> Node {
    let mut node = create_new_node(token_data);

    let next_token_data = get_token(lex);

    if next_token_data.token == Token::Number {
        let number_node = parse_number(next_token_data, lex);
        node.right = Some(Box::new(number_node));
    } else {
        panic!("current invalid input for after plus");
    }

    return node;
}

fn parse_divide(token_data: TokenData, lex: & mut Lexer<'_, Token>) -> Node {
    let mut node = create_new_node(token_data);

    let next_token_data = get_token(lex);

    if next_token_data.token == Token::Number {
        let number_node = parse_number(next_token_data, lex);
        node.right = Some(Box::new(number_node));
    } else {
        panic!("current invalid input for after divide");
    }

    return node;
}

fn parse_multiply(token_data: TokenData, lex: & mut Lexer<'_, Token>) -> Node {
    let mut node = create_new_node(token_data);

    let next_token_data = get_token(lex);

    if next_token_data.token == Token::Number {
        let number_node = parse_number(next_token_data, lex);
        node.right = Some(Box::new(number_node));
    } else {
        panic!("current invalid input for after multiply");
    }

    return node;
}

fn parse_semi(token_data: TokenData, lex: & mut Lexer<'_, Token>) -> Node {
    let node = create_new_node(token_data);

    return node;
}

fn parse_number(token_data: TokenData, lex: & mut Lexer<'_, Token>) -> Node {
    let node = create_new_node(token_data);

    let next_token_data = get_token(lex);

    let mut node_to_return = match next_token_data.token {
        Token::Minus => parse_minus(next_token_data, lex),
        Token::Plus => parse_plus(next_token_data, lex),
        Token::Divide => parse_divide(next_token_data, lex),
        Token::Multiply => parse_multiply(next_token_data, lex),
        Token::SemiColon => parse_semi(next_token_data, lex),
        _ => panic!("invalid input for parse number: {}", next_token_data.slice)
    };

    node_to_return.left = Some(Box::new(node));

    return node_to_return;
}

fn parse_equals(token_data: TokenData, lex: & mut Lexer<'_, Token>) -> Node {
    let mut node = create_new_node(token_data);

    let next_token_data = get_token(lex);

    if next_token_data.token == Token::Minus {
        let minus_node = parse_minus(next_token_data, lex);
        node.right = Some(Box::new(minus_node));
    } else if next_token_data.token == Token::Number {
        let number_node = parse_number(next_token_data, lex);
        node.right = Some(Box::new(number_node));
    }

    return node;
}

fn parse_variable(token_data: TokenData, lex: & mut Lexer<'_, Token>) -> Node {
    if token_data.slice.chars().next().unwrap().is_uppercase() {
        panic!("uppercase is not allowed for a variable");
    }

    let node = create_new_node(token_data);

    let next_token_data = get_token(lex);

    if next_token_data.token != Token::Equals {

        panic!("next char is not equals!");
    }

    let mut equals_node = parse_equals(next_token_data, lex);

    equals_node.left = Some(Box::new(node));

    return equals_node;
}

fn program(lex: & mut Lexer<'_, Token>) -> Node {
    let token_data = get_token(lex);

    let node_token_data = TokenData {
        token: Token::Program,
        slice: "".to_string()
    };

    let mut node = create_new_node(node_token_data);

    if token_data.token == Token::Text {
        let next_node = parse_variable(token_data, lex);

        node.next = Some(Box::new(next_node));

    } else {
        panic!("unsupported character. only variables supported for now");
    }

    return node;
}

fn main() {
    let text = match fs::read_to_string("./test.bro") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file {:?}", error)
    };

    let mut lex = Token::lexer(&text);
    let root = program(&mut lex);
    let ast_root = action::parse(&root);
    println!("{:#?}", root);
}


