use std::io::{self, Stdin, Stdout, Write};

use crate::{lexer::Lexer, token::Token};

pub fn start() {
    let stdin: Stdin = io::stdin();
    let mut stdout: Stdout = io::stdout();
    let mut input = String::new();

    print!("This is the Rusty Monkey REPL, feel free to type in commands!\n");

    loop {
        print!(">>> ");
        stdout.flush().expect("Failed to flush output to stdout!");
        stdin
            .read_line(&mut input)
            .expect("Failed to read line from stdin!");

        let mut lexer = Lexer::new(input.trim());
        let mut tok = lexer.next_token();

        while tok != Token::Eof {
            print!("{:?}\n", tok);
            tok = lexer.next_token();
        }

        input.clear();
    }
}
