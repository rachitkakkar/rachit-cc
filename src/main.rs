use std::{env, fs};

use rachit_cc::lexer::{lex, TokenType};

fn compile(file_path: &String) -> Result<(), String> {
  match fs::read_to_string(file_path) {
    Err(msg) => return Err(msg.to_string()),
    Ok(contents) => {
      lex(&contents); // To-do, handle unused `Result`
      return Ok(())
    }
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("Please provide a valid RC file")
  } else {
    let file_path: &String = &args[1];
    compile(file_path); // To-do, handle unused `Result`
  }
}
