use std::{
    env,
    io::{stdin, stdout},
};

mod ast;
mod lexer;
mod parser;
mod repl;
mod token;
fn main() {
    let username = env::var("USER").unwrap_or("man/woman".to_owned());
    println!("Hello {username}! This is the Monkey programming language (Rust implementation)!");
    println!("Feel free to type in commands\n");

    repl::start(&mut stdin(), &mut stdout());
}
