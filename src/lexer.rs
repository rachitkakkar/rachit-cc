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

      _ => return Err(format!("Unrecognized character {} at line {}", ch, line_num))
    }

    position += 1;
  }

  return Ok(tokens);
}

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
}
