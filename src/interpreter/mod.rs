use std::fs;
use std::io;
use std::io::Write;
use std::process;

use crate::lexer::lexer::Lexer;
use crate::parser::parser::{Ast, Parser};

pub fn execute_line(line: &str) {
    if line.trim().is_empty() || line.trim().starts_with('#') {
        return;
    }

    let mut lexer = Lexer::new(line);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    match ast {
        Ast::Yap(msg) => {
            println!("{}", msg);
        }
        Ast::NoOp => {
            println!("Holly Slop... Unc don't know how to script 😭");
        }
    }
}

pub fn run_script(file_name: &str) {
    match fs::read_to_string(file_name) {
        Ok(content) => {
            println!("Slop-Shell v0.1 - Slopping since 2026\n");
            for (_i, line) in content.lines().enumerate() {
                execute_line(line);
            }
        }
        Err(e) => {
            eprintln!("Failed to read script '{}': {}", file_name, e);
            process::exit(1);
        }
    }
}

pub fn run_repl() {
    println!("Slop-Shell v0.1 - Slopping since 2026");
    println!("Type 'exit' to cope and seethe.\n");

    loop {
        print!("slop> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        if input == "exit" || input == "quit" {
            println!("Bye Unc 👋");
            break;
        }

        execute_line(input);
    }
}
