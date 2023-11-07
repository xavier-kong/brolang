use std::fs;

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
enum Token {
    // Tokens can be literal strings, of any length.
    #[token("shake")]
    Shake,

    /* #[token("{")]*/
    /*OpenCurly,*/

    #[token("{")]
    OpenCurlyBracket,

    #[token("}")]
    CloseCurlyBracket,

    #[token("[")]
    OpenSquareBracket,

    #[token("]")]
    CloseSquareBracket,

    #[token("(")]
    OpenParen,

    #[token(")")]
    CloseParen,

    #[token(";")]
    SemiColon,

    #[token(":")]
    Colon,

    #[token("=")]
    Equals,

    #[token("//")]
    CommentBegin,

    // Or regular expressions.
    #[regex("[a-zA-Z]+")]
    Text,
}

fn main() {
    let text = match fs::read_to_string("./design.bro") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file {:?}", error)
    };

    let mut lex = Token::lexer(&text);

    let mut token = lex.next();

    while token != None  {
        if let Some(tok) = &token {
            match tok {
                Ok(_) => (),
                Err(_) => { println!("{:?} {:?}", tok, lex.slice()) }
            }
            ;
        }
        token = lex.next();
    }
}


