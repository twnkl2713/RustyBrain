mod lexer;
mod parser;
mod interpreter;

use std::env;
use std::fs;
use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: brainfuck_interpreter <input.bf> [debug]");
        std::process::exit(1);
    }

    let filename = &args[1];
    let debug = args.len() == 3 && args[2] == "debug";

    let code = fs::read_to_string(filename)?;
    let tokens = lexer::tokenize(&code);
    let (program, _) = parser::parse(&tokens)?;
    interpreter::interpret(&program, debug)?;
    Ok(())
}