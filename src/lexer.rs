// Token Type(Token Lexeme/Literal)
#[derive(Debug, PartialEq)]
pub enum TokenType {
  Def(String),
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

pub struct Token {
  pub t_tok: TokenType,
  pub line_num: u32,
  pub position: u32,
}

impl Token {
  pub fn new(t_tok: TokenType, line_num: u32, position: u32) -> Self {
    Token { t_tok, line_num, position }
  }
}

pub fn lex(input: &String) -> Result<Vec<Token>, String>  {
  let mut tokens: Vec<Token> = Vec::new();

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
      '(' => tokens.push(Token::new(TokenType::OpenParen('('), line_num, position)),
      ')' => tokens.push(Token::new(TokenType::CloseParen(')'), line_num, position)),
      '{' => tokens.push(Token::new(TokenType::OpenCurly('{'), line_num, position)),
      '}' => tokens.push(Token::new(TokenType::CloseCurly('}'), line_num, position)),
      '[' => tokens.push(Token::new(TokenType::OpenBracket('['), line_num, position)),
      ']' => tokens.push(Token::new(TokenType::CloseBracket(']'), line_num, position)),

      // Handle single-character operators and punctuation
      '+' => tokens.push(Token::new(TokenType::Plus('+'), line_num, position)),
      '-' => tokens.push(Token::new(TokenType::Minus('-'), line_num, position)),
      '*' => tokens.push(Token::new(TokenType::Times('*'), line_num, position)),
      '/' => tokens.push(Token::new(TokenType::Divide('/'), line_num, position)),
      ',' => tokens.push(Token::new(TokenType::Comma(','), line_num, position)),
      ';' => tokens.push(Token::new(TokenType::Semicolon(';'), line_num, position)),

      // Handle two-character operators
      '=' => {
        if let Some(next_ch) = it.peek() {
          match next_ch {
            '=' => tokens.push(Token::new(TokenType::EqualEqual("==".to_string()), line_num, position)),
            _ => {
              tokens.push(Token::new(TokenType::Equal('='), line_num, position));
              continue;
            }
          }
        }
      },
      '>' => {
        if let Some(next_ch) = it.peek() {
          match next_ch {
            '=' => tokens.push(Token::new(TokenType::GreaterThanEqual(">=".to_string()), line_num, position)),
            _ => {
              tokens.push(Token::new(TokenType::GreaterThan('>'), line_num, position));
              continue;
            }
          }
        }
      }
      '<' => {
        if let Some(next_ch) = it.peek() {
          match next_ch {
            '=' => tokens.push(Token::new(TokenType::LessThanEqual("==".to_string()), line_num, position)),
            _ => {
              tokens.push(Token::new(TokenType::LessThan('<'), line_num, position));
              continue;
            }
          }
        }
      },

      // Ignore comments (skip until the end of the line)
      '#' => {
        while let Some(ch) = it.next() {
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
          "def" => tokens.push(Token::new(TokenType::Def(identifier), line_num, position)),
          "if" => tokens.push(Token::new(TokenType::If(identifier), line_num, position)),
          "else" => tokens.push(Token::new(TokenType::Else(identifier), line_num, position)),
          "while" => tokens.push(Token::new(TokenType::While(identifier), line_num, position)),
          "return" => tokens.push(Token::new(TokenType::Return(identifier), line_num, position)),
          "break" => tokens.push(Token::new(TokenType::Break(identifier), line_num, position)),
          "continue" => tokens.push(Token::new(TokenType::Continue(identifier), line_num, position)),
          "true" => tokens.push(Token::new(TokenType::True(identifier), line_num, position)),
          "false" => tokens.push(Token::new(TokenType::False(identifier), line_num, position)),
          _ => tokens.push(Token::new(TokenType::Identifier(identifier), line_num, position)),
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
          Ok(num) => tokens.push(Token::new(TokenType::Number(num), line_num, position)),
          Err(_) => return Err(format!("Invalid number {} at line {}", num_str, line_num)),
        }
      },
      _ => return Err(format!("Unrecognized character {} at line {}", ch, line_num))
    }

    position += 1;
  }

  return Ok(tokens);
}

// To-do tests
// 1. Simple arithmetic -> 4+5*6
// 2. Test with function definition, function call, and binary operations with precedence -> 
// def test(x, y) { 
//   (1+2+x)*(test(x, 2)+(1+2))
// }
// def foo(x) {
//  (1+2+x)*(x+(1+2))
// }
//
// x = foo(1)
// test(x, 5)
// 3. If statement, while statement, and top-level statement test
// def bar(x) {
//   while (x < 1) {
//     1 + 5 * bar(1)
//   }
// }

// def foo(x) {
//   if (x<1) { 
//     foo(1)
//   }
//   else {
//     bar(x)
//   }
// }
// foo(1)
// 4. Example program
// # Compute the x'th fibonacci number recursively.
// def fib(x) {
//   if (x < 3) {
//     1
//   }
//   else {
//     fib(x-1)+fib(x-2)
//   }
// }

// # This expression will compute the 40th number.
// fib(40)

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn lex_groupings() {
    let source: String = "[{( )}]".to_string();
    let result = lex(&source);
    assert_eq!(result.is_ok(), true);
    let tokens: Vec<Token> = result.unwrap();
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].t_tok, TokenType::OpenBracket('['));
    assert_eq!(tokens[1].t_tok, TokenType::OpenCurly('{'));
    assert_eq!(tokens[2].t_tok, TokenType::OpenParen('('));
    assert_eq!(tokens[3].t_tok, TokenType::CloseParen(')'));
    assert_eq!(tokens[4].t_tok, TokenType::CloseCurly('}'));
    assert_eq!(tokens[5].t_tok, TokenType::CloseBracket(']'));
  }

  #[test]
  fn lex_comments() {
    // Comment at end of file
    let mut source: String = "+ \n # This is a comment".to_string();
    let mut result = lex(&source);
    assert_eq!(result.is_ok(), true);
    let mut tokens: Vec<Token> = result.unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].t_tok, TokenType::Plus('+'));

    // Comment in the middle of a file
    source = "+ \n # This is a comment \n +".to_string();
    result = lex(&source);
    assert_eq!(result.is_ok(), true);
    tokens = result.unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].t_tok, TokenType::Plus('+'));
    assert_eq!(tokens[1].t_tok, TokenType::Plus('+'));
  }

  #[test]
  fn lex_unrecognized_char() {
    let source: String = "5 + 5?".to_string(); // To-do, replace with numbers
    let result = lex(&source);
    assert_eq!(result.is_ok(), false);
  }
}
