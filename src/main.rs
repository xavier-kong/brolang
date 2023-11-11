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
    Constant
}

struct Node {
    data: Token,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    next: Option<Box<Node>>
}

fn get_token(mut lex: Lexer<'_, Token>) -> Token {
    let curr = match lex.next() {
        Some(val) => val,
        None => panic!("panicking!")
    };
    let token = match curr {
        Ok(val) => val,
        Err(e) => panic!("{:?}", e)
    };
    return token;
}

fn program(lex: Lexer<'_, Token>) -> &Node {
    let token = get_token(lex);

}

fn main() {
    let text = match fs::read_to_string("./design.bro") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file {:?}", error)
    };

    let lex = Token::lexer(&text);
    let root = program(lex);
}


