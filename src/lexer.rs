// Token Type(Token Lexeme/Literal)
#[derive(Debug, PartialEq)]
pub enum Token {
  Def(String),
  Let(String),
  If(String),
  Else(String),
  While(String),
  Return(String),
  Break(String),
  Continue(String),
  True(String),
  False(String),

  OpenParen(char),
  CloseParen(char),
  OpenCurly(char),
  CloseCurly(char),
  OpenBracket(char),
  CloseBracket(char),

  Equal(char),
  EqualEqual(String),
  LessThan(char),
  GreaterThan(char),
  LessThanEqual(String),
  GreaterThanEqual(String),
  Plus(char),
  Minus(char),
  Times(char),
  Divide(char),
  Comma(char),
  Semicolon(char),

  Number(f64),
  Identifier(String),
}

pub struct LoggedToken {
  pub token: Token,
  pub line_num: u32,
  pub position: u32,
}

impl LoggedToken {
  pub fn new(t_tok: Token, line_num: u32, position: u32) -> Self {
    LoggedToken { token: t_tok, line_num, position }
  }
}

pub fn lex(input: &String) -> Result<Vec<LoggedToken>, String>  {
  let mut tokens: Vec<LoggedToken> = Vec::new();

  // Position in file
  let mut it: std::iter::Peekable<std::str::Chars<'_>> = input.chars().peekable();
  let mut line_num: u32 = 0;
  let mut position: u32 = 0;

  while let Some(ch) = it.next() {
    match ch {
      // Handle whitespace and new lines
      ' ' | '\t' => continue,
      '\n' => line_num += 1,

      // Handle various single-character tokens like parentheses, brackets, and operators
      '(' => tokens.push(LoggedToken::new(Token::OpenParen('('), line_num, position)),
      ')' => tokens.push(LoggedToken::new(Token::CloseParen(')'), line_num, position)),
      '{' => tokens.push(LoggedToken::new(Token::OpenCurly('{'), line_num, position)),
      '}' => tokens.push(LoggedToken::new(Token::CloseCurly('}'), line_num, position)),
      '[' => tokens.push(LoggedToken::new(Token::OpenBracket('['), line_num, position)),
      ']' => tokens.push(LoggedToken::new(Token::CloseBracket(']'), line_num, position)),

      // Handle single-character operators and punctuation
      '+' => tokens.push(LoggedToken::new(Token::Plus('+'), line_num, position)),
      '-' => tokens.push(LoggedToken::new(Token::Minus('-'), line_num, position)),
      '*' => tokens.push(LoggedToken::new(Token::Times('*'), line_num, position)),
      '/' => tokens.push(LoggedToken::new(Token::Divide('/'), line_num, position)),
      ',' => tokens.push(LoggedToken::new(Token::Comma(','), line_num, position)),
      ';' => tokens.push(LoggedToken::new(Token::Semicolon(';'), line_num, position)),

      // Handle two-character operators
      '=' => {
        if let Some(next_ch) = it.peek() {
          match next_ch {
            '=' => {
              tokens.push(LoggedToken::new(Token::EqualEqual("==".to_string()), line_num, position));
              it.next();
              position += 1;
            },
            _ => {
              tokens.push(LoggedToken::new(Token::Equal('='), line_num, position));
              continue;
            }
          }
        }
      },
      '>' => {
        if let Some(next_ch) = it.peek() {
          match next_ch {
            '=' => {
              tokens.push(LoggedToken::new(Token::GreaterThanEqual(">=".to_string()), line_num, position));
              it.next();
              position += 1;
            },
            _ => {
              tokens.push(LoggedToken::new(Token::GreaterThan('>'), line_num, position));
              continue;
            }
          }
        }
      }
      '<' => {
        if let Some(next_ch) = it.peek() {
          match next_ch {
            '=' => { 
              tokens.push(LoggedToken::new(Token::LessThanEqual("<=".to_string()), line_num, position));
              it.next();
              position += 1;
            },
            _ => {
              tokens.push(LoggedToken::new(Token::LessThan('<'), line_num, position));
              continue;
            }
          }
        }
      },

      // Ignore comments (skip until the end of the line)
      '#' => {
        while let Some(ch) = it.next() {
          position += 1;
          if ch == '\n' {
            line_num += 1;
            break;
          }
        }
      },

      // Handle keywords (def, if, else, while, etc.)
      'a'..='z' | 'A'..='Z' => {
        let mut identifier = ch.to_string();
        while let Some(next_ch) = it.peek() {
          if next_ch.is_alphanumeric() || *next_ch == '_' {
            identifier.push(it.next().unwrap());
            position += 1;
          } else {
            break;
          }
        }

        // Check if it's a keyword
        match identifier.as_str() {
          "def" => tokens.push(LoggedToken::new(Token::Def(identifier), line_num, position)),
          "let" => tokens.push(LoggedToken::new(Token::Let(identifier), line_num, position)),
          "if" => tokens.push(LoggedToken::new(Token::If(identifier), line_num, position)),
          "else" => tokens.push(LoggedToken::new(Token::Else(identifier), line_num, position)),
          "while" => tokens.push(LoggedToken::new(Token::While(identifier), line_num, position)),
          "return" => tokens.push(LoggedToken::new(Token::Return(identifier), line_num, position)),
          "break" => tokens.push(LoggedToken::new(Token::Break(identifier), line_num, position)),
          "continue" => tokens.push(LoggedToken::new(Token::Continue(identifier), line_num, position)),
          "true" => tokens.push(LoggedToken::new(Token::True(identifier), line_num, position)),
          "false" => tokens.push(LoggedToken::new(Token::False(identifier), line_num, position)),
          _ => tokens.push(LoggedToken::new(Token::Identifier(identifier), line_num, position)),
        }
      },

      // Handle numbers (floating point or integers)
      '0'..='9' => {
        let mut num_str = ch.to_string();
        while let Some(next_ch) = it.peek() {
          if next_ch.is_digit(10) || *next_ch == '.' {
            num_str.push(it.next().unwrap());
            position += 1;
          } else {
            break;
          }
        }

        // Convert to number
        match num_str.parse::<f64>() {
          Ok(num) => tokens.push(LoggedToken::new(Token::Number(num), line_num, position)),
          Err(_) => return Err(format!("Invalid number {} at line {}", num_str, line_num)),
        }
      },
      _ => return Err(format!("Unrecognized character {} at line {}", ch, line_num))
    }

    position += 1;
  }

  return Ok(tokens);
}

#[cfg(test)]
mod tests {
  use super::*;

  // Specific operator tests
  #[test]
  fn lex_variable_assignments() {
    let source: String = "let x = 5.237 \n x = 6".to_string();
    let result = lex(&source);
    assert_eq!(result.is_ok(), true);
    
    let tokens: Vec<LoggedToken> = result.unwrap();
    assert_eq!(tokens.len(), 7);

    assert_eq!(tokens[0].token, Token::Let("let".to_string()));
    assert_eq!(tokens[1].token, Token::Identifier("x".to_string()));
    assert_eq!(tokens[2].token, Token::Equal('='));
    assert_eq!(tokens[3].token, Token::Number(5.237));
    assert_eq!(tokens[4].token, Token::Identifier("x".to_string()));
    assert_eq!(tokens[5].token, Token::Equal('='));
    assert_eq!(tokens[6].token, Token::Number(6.0));
  }
  
  #[test]
  fn lex_binary_operations() {
    let source: String = "4 + 5 * 6 \n 7.3 / 3.46 - 5.2".to_string();
    let result = lex(&source);
    assert_eq!(result.is_ok(), true);
    
    let tokens: Vec<LoggedToken> = result.unwrap();
    assert_eq!(tokens.len(), 10);

    assert_eq!(tokens[0].token, Token::Number(4.0));
    assert_eq!(tokens[1].token, Token::Plus('+'));
    assert_eq!(tokens[2].token, Token::Number(5.0));
    assert_eq!(tokens[3].token, Token::Times('*'));
    assert_eq!(tokens[4].token, Token::Number(6.0));
    assert_eq!(tokens[5].token, Token::Number(7.3));
    assert_eq!(tokens[6].token, Token::Divide('/'));
    assert_eq!(tokens[7].token, Token::Number(3.46));
    assert_eq!(tokens[8].token, Token::Minus('-'));
    assert_eq!(tokens[9].token, Token::Number(5.2));
  }

  #[test]
  fn lex_comparison_operations() {
    let source: String = "8 == 3 + 4 * 5 \n 8 >= 3 + 4 * 5 \n 8 <= 3 + 4 * 5 == false \n 8 >= 3 + 4 * 5 == true".to_string();
    let result = lex(&source);
    assert_eq!(result.is_ok(), true);
    
    let tokens: Vec<LoggedToken> = result.unwrap();
    assert_eq!(tokens.len(), 32);

    assert_eq!(tokens[0].token, Token::Number(8.0));
    assert_eq!(tokens[1].token, Token::EqualEqual("==".to_string()));
    assert_eq!(tokens[2].token, Token::Number(3.0));
    assert_eq!(tokens[3].token, Token::Plus('+'));
    assert_eq!(tokens[4].token, Token::Number(4.0));
    assert_eq!(tokens[5].token, Token::Times('*'));
    assert_eq!(tokens[6].token, Token::Number(5.0));

    assert_eq!(tokens[7].token, Token::Number(8.0));
    assert_eq!(tokens[8].token, Token::GreaterThanEqual(">=".to_string()));
    assert_eq!(tokens[9].token, Token::Number(3.0));
    assert_eq!(tokens[10].token, Token::Plus('+'));
    assert_eq!(tokens[11].token, Token::Number(4.0));
    assert_eq!(tokens[12].token, Token::Times('*'));
    assert_eq!(tokens[13].token, Token::Number(5.0));

    assert_eq!(tokens[14].token, Token::Number(8.0));
    assert_eq!(tokens[15].token, Token::LessThanEqual("<=".to_string()));
    assert_eq!(tokens[16].token, Token::Number(3.0));
    assert_eq!(tokens[17].token, Token::Plus('+'));
    assert_eq!(tokens[18].token, Token::Number(4.0));
    assert_eq!(tokens[19].token, Token::Times('*'));
    assert_eq!(tokens[20].token, Token::Number(5.0));
    assert_eq!(tokens[21].token, Token::EqualEqual("==".to_string()));
    assert_eq!(tokens[22].token, Token::False("false".to_string()));

    assert_eq!(tokens[23].token, Token::Number(8.0));
    assert_eq!(tokens[24].token, Token::GreaterThanEqual(">=".to_string()));
    assert_eq!(tokens[25].token, Token::Number(3.0));
    assert_eq!(tokens[26].token, Token::Plus('+'));
    assert_eq!(tokens[27].token, Token::Number(4.0));
    assert_eq!(tokens[28].token, Token::Times('*'));
    assert_eq!(tokens[29].token, Token::Number(5.0));
    assert_eq!(tokens[30].token, Token::EqualEqual("==".to_string()));
    assert_eq!(tokens[31].token, Token::True("true".to_string()));
  }

  // Smaller, not syntactically correct tests
  #[test]
  fn lex_unrecognized_char() {
    let source: String = "5 + 5?".to_string();
    let result = lex(&source);
    assert_eq!(result.is_ok(), false);
  }
  
  #[test]
  fn lex_comments() {
    // Comment at end of file
    let mut source: String = "+ \n # This is a comment".to_string();
    let mut result = lex(&source);
    assert_eq!(result.is_ok(), true);
    let mut tokens: Vec<LoggedToken> = result.unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token, Token::Plus('+'));

    // Comment in the middle of a file
    source = "+ \n # This is a comment \n +".to_string();
    result = lex(&source);
    assert_eq!(result.is_ok(), true);
    tokens = result.unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token, Token::Plus('+'));
    assert_eq!(tokens[1].token, Token::Plus('+'));
  }
  
  #[test]
  fn lex_groupings() {
    let source: String = "[{( )}]".to_string();
    let result = lex(&source);
    assert_eq!(result.is_ok(), true);
    let tokens: Vec<LoggedToken> = result.unwrap();
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].token, Token::OpenBracket('['));
    assert_eq!(tokens[1].token, Token::OpenCurly('{'));
    assert_eq!(tokens[2].token, Token::OpenParen('('));
    assert_eq!(tokens[3].token, Token::CloseParen(')'));
    assert_eq!(tokens[4].token, Token::CloseCurly('}'));
    assert_eq!(tokens[5].token, Token::CloseBracket(']'));
  }
  
  #[test]
  fn lex_empty_input() {
    let source: String = "".to_string();
    let result = lex(&source);
    assert_eq!(result.is_ok(), true);
    let tokens: Vec<LoggedToken> = result.unwrap();
    assert_eq!(tokens.len(), 0);
  }

  #[test]
  fn lex_invalid_number_format() {
    let source: String = "5.2.3".to_string();
    let result = lex(&source);
    assert_eq!(result.is_ok(), false);
  }  

  // Example program tests
  #[test]
  fn lex_fibonacci_program() {
    let source: String = r#"
# Compute the x'th Fibonacci number.
def fib(x) {
  if (x < 3) {
    return 1
  }
  else {
    return fib(x-1) + fib(x-2)
  }
}

# This expression will compute the 40th number.
fib(40)
"#.to_string();
    let result = lex(&source);
    assert_eq!(result.is_ok(), true);

    let tokens: Vec<LoggedToken> = result.unwrap();
    assert_eq!(tokens.len(), 38);
    
    // Test for the function definition of fib
    assert_eq!(tokens[0].token, Token::Def("def".to_string()));
    assert_eq!(tokens[1].token, Token::Identifier("fib".to_string()));
    assert_eq!(tokens[2].token, Token::OpenParen('('));
    assert_eq!(tokens[3].token, Token::Identifier("x".to_string()));
    assert_eq!(tokens[4].token, Token::CloseParen(')'));
    assert_eq!(tokens[5].token, Token::OpenCurly('{'));
    
    // Test for the if condition and the return statement
    assert_eq!(tokens[6].token, Token::If("if".to_string()));
    assert_eq!(tokens[7].token, Token::OpenParen('('));
    assert_eq!(tokens[8].token, Token::Identifier("x".to_string()));
    assert_eq!(tokens[9].token, Token::LessThan('<'));
    assert_eq!(tokens[10].token, Token::Number(3.0));
    assert_eq!(tokens[11].token, Token::CloseParen(')'));
    assert_eq!(tokens[12].token, Token::OpenCurly('{'));
    assert_eq!(tokens[13].token, Token::Return("return".to_string()));
    assert_eq!(tokens[14].token, Token::Number(1.0));
    assert_eq!(tokens[15].token, Token::CloseCurly('}'));
    
    // Test for the else block and recursive call
    assert_eq!(tokens[16].token, Token::Else("else".to_string()));
    assert_eq!(tokens[17].token, Token::OpenCurly('{'));
    assert_eq!(tokens[18].token, Token::Return("return".to_string()));
    assert_eq!(tokens[19].token, Token::Identifier("fib".to_string()));
    assert_eq!(tokens[20].token, Token::OpenParen('('));
    assert_eq!(tokens[21].token, Token::Identifier("x".to_string()));
    assert_eq!(tokens[22].token, Token::Minus('-'));
    assert_eq!(tokens[23].token, Token::Number(1.0));
    assert_eq!(tokens[24].token, Token::CloseParen(')'));
    assert_eq!(tokens[25].token, Token::Plus('+'));
    assert_eq!(tokens[26].token, Token::Identifier("fib".to_string()));
    assert_eq!(tokens[27].token, Token::OpenParen('('));
    assert_eq!(tokens[28].token, Token::Identifier("x".to_string()));
    assert_eq!(tokens[29].token, Token::Minus('-'));
    assert_eq!(tokens[30].token, Token::Number(2.0));
    assert_eq!(tokens[31].token, Token::CloseParen(')'));
    assert_eq!(tokens[32].token, Token::CloseCurly('}'));
    assert_eq!(tokens[33].token, Token::CloseCurly('}'));
    
    // Test for the function call at the end
    assert_eq!(tokens[34].token, Token::Identifier("fib".to_string()));
    assert_eq!(tokens[35].token, Token::OpenParen('('));
    assert_eq!(tokens[36].token, Token::Number(40.0));
    assert_eq!(tokens[37].token, Token::CloseParen(')'));
  }

  #[test]
  fn lex_fibonacci_with_while_loop() {
    let source: String = r#"
# Compute Fibonacci iteratively.
def fib(x) {
  let a = 0
  let b = 1
  while (x > 0) {
    let temp = a
    a = b
    b = temp + b
    x = x - 1
  }
  return a
}

# Compute the 10th Fibonacci number.
fib(10)
"#.to_string();

    let result = lex(&source);
    assert_eq!(result.is_ok(), true);

    let tokens: Vec<LoggedToken> = result.unwrap();
    assert_eq!(tokens.len(), 46);
    
    // Test for the function definition of fib with the while loop
    assert_eq!(tokens[0].token, Token::Def("def".to_string()));
    assert_eq!(tokens[1].token, Token::Identifier("fib".to_string()));
    assert_eq!(tokens[2].token, Token::OpenParen('('));
    assert_eq!(tokens[3].token, Token::Identifier("x".to_string()));
    assert_eq!(tokens[4].token, Token::CloseParen(')'));
    assert_eq!(tokens[5].token, Token::OpenCurly('{'));
    
    // Test for variable assignments
    assert_eq!(tokens[6].token, Token::Let("let".to_string()));
    assert_eq!(tokens[7].token, Token::Identifier("a".to_string()));
    assert_eq!(tokens[8].token, Token::Equal('='));
    assert_eq!(tokens[9].token, Token::Number(0.0));
    
    assert_eq!(tokens[10].token, Token::Let("let".to_string()));
    assert_eq!(tokens[11].token, Token::Identifier("b".to_string()));
    assert_eq!(tokens[12].token, Token::Equal('='));
    assert_eq!(tokens[13].token, Token::Number(1.0));
    
    // Test for the while loop condition
    assert_eq!(tokens[14].token, Token::While("while".to_string()));
    assert_eq!(tokens[15].token, Token::OpenParen('('));
    assert_eq!(tokens[16].token, Token::Identifier("x".to_string()));
    assert_eq!(tokens[17].token, Token::GreaterThan('>'));
    assert_eq!(tokens[18].token, Token::Number(0.0));
    assert_eq!(tokens[19].token, Token::CloseParen(')'));
    assert_eq!(tokens[20].token, Token::OpenCurly('{'));
    
    // Test for variable assignments inside the loop
    assert_eq!(tokens[21].token, Token::Let("let".to_string()));
    assert_eq!(tokens[22].token, Token::Identifier("temp".to_string()));
    assert_eq!(tokens[23].token, Token::Equal('='));
    assert_eq!(tokens[24].token, Token::Identifier("a".to_string()));
    
    assert_eq!(tokens[25].token, Token::Identifier("a".to_string()));
    assert_eq!(tokens[26].token, Token::Equal('='));
    assert_eq!(tokens[27].token, Token::Identifier("b".to_string()));
    
    assert_eq!(tokens[28].token, Token::Identifier("b".to_string()));
    assert_eq!(tokens[29].token, Token::Equal('='));
    assert_eq!(tokens[30].token, Token::Identifier("temp".to_string()));
    assert_eq!(tokens[31].token, Token::Plus('+'));
    assert_eq!(tokens[32].token, Token::Identifier("b".to_string()));
    
    assert_eq!(tokens[33].token, Token::Identifier("x".to_string()));
    assert_eq!(tokens[34].token, Token::Equal('='));
    assert_eq!(tokens[35].token, Token::Identifier("x".to_string()));
    assert_eq!(tokens[36].token, Token::Minus('-'));
    assert_eq!(tokens[37].token, Token::Number(1.0));
    assert_eq!(tokens[38].token, Token::CloseCurly('}'));
    
    // Test for the return statement
    assert_eq!(tokens[39].token, Token::Return("return".to_string()));
    assert_eq!(tokens[40].token, Token::Identifier("a".to_string()));
    assert_eq!(tokens[41].token, Token::CloseCurly('}'));

    // Test for the function call
    assert_eq!(tokens[42].token, Token::Identifier("fib".to_string()));
    assert_eq!(tokens[43].token, Token::OpenParen('('));
    assert_eq!(tokens[44].token, Token::Number(10.0));
    assert_eq!(tokens[45].token, Token::CloseParen(')'));
  }
}
