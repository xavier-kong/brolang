use std::fs;

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
enum Token {
    // Tokens can be literal strings, of any length.
    #[token("shake")]
    Shake,

    #[token("{")]
    OpenCurly,

    #[token("}")]
    CloseCurly,

    #[token(":")]
    Colon,

    // Or regular expressions.
    #[regex("[a-zA-Z]+")]
    Text,
}

fn main() {
    let open_text_res = fs::read_to_string("../design.bro");

    let text = match open_text_res {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file {:?}", error)
    };

    let mut lex = Token::lexer(&text);

    while lex.next().is_some() {
        println!("{}", lex.slice());
    }
}


