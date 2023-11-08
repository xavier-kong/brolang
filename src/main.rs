use std::fs;

use logos::Logos;

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

fn main() {
    let text = match fs::read_to_string("./design.bro") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file {:?}", error)
    };

    let mut lex = Token::lexer(&text);

    println!("{:?}", lex);

    let mut token = lex.next();

    while token != None  {
        if let Some(tok) = &token {
            match tok {
                Ok(val) => { println!("TOK {:?} {:?}", val, lex.slice()) },
                Err(_) => { println!("ERROR {:?} {:?}", tok, lex.slice()) }
            }
            ;
        }
        token = lex.next();
    }
}


