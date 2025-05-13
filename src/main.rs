use std::{env, fs};

use rachit_cc::{
  lexer::lex, 
  parser::Parser,
};

fn compile(file_path: &String) -> Result<(), String> {
  match fs::read_to_string(file_path) {
    Err(msg) => return Err(msg.to_string()),
    Ok(contents) => {
      let tokens = lex(&contents)?;
      let parser = Parser { tokens };
      parser.parse()?;
      return Ok(())
    }
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("Please provide a valid SIL file")
  } else {
    let file_path: &String = &args[1];
    compile(file_path); // To-do, handle unused `Result`
  }
}
