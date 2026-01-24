mod ast;
mod token;
mod lexer;
mod parser;
mod interpreter;
mod engine;
mod functions;

use std::env;
use std::fs;
use std::io::{self, Read};
use crate::{lexer::Lexer, parser::Parser, engine::Engine};

fn wait_for_keypress() {
    println!("\nPress any key to exit...");
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}

fn main() {
    let file = match env::args().nth(1) {
        Some(f) => f,
        None => {
            return;
        }
    };

    let src = match fs::read_to_string(&file) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to read {}: {}", file, e);
            wait_for_keypress();
            return;
        }
    };

    let mut lexer = Lexer::new(&src);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);

    match parser.parse() {
        Ok(ast) => {
            let engine = Engine::new();
            if let Err(e) = engine.run(ast) {
                println!("Runtime error: {}", e);
                wait_for_keypress();
            }
        }
        Err(e) => {
            println!("Syntax error: {}", e);
            wait_for_keypress();
        }
    }
}