use std::{fs, path::Path};

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
enum Token {
    // Tokens can be literal strings, of any length.
    #[token("shake")]
    Shake,

    /* #[token("{")]*/
    /*OpenCurly,*/

    #[token("{|}")]
    Curly,

    #[token(":")]
    Colon,

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

    while let token = lex.next() {
        println!("{:?}", token);
    }
}


