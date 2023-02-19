use std::io::{BufRead, BufReader, Read, Write};

use crate::{lexer::Lexer, token::TokenType};

const PROMPT: &str = ">> ";

pub(crate) fn start<Reader, Writer>(is: &mut Reader, outer: &mut Writer)
where
    Reader: Read,
    Writer: Write,
{
    let mut scanner = BufReader::new(is);

    loop {
        write!(outer, "{PROMPT}").expect("Failed to writer distance");
        outer.flush().unwrap();

        let mut text: String = "".to_string();

        let scanned = scanner.read_line(&mut text);
        if scanned.is_err() {
            eprintln!("Failed read input to buffer: {}", scanned.err().unwrap());
            continue;
        }

        let mut lexer = Lexer::new(&text);
        loop {
            let tok = lexer.next_token();
            if tok.typee == TokenType::Eof {
                break;
            }
            println!("{tok:?}");
        }
    }
}
