use std::env;

pub mod interpreter;
pub mod lexer;
pub mod parser;

use interpreter::{run_repl, run_script};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let script_path = &args[1];
        run_script(script_path);
    } else {
        run_repl();
    }
}
