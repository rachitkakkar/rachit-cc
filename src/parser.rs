use crate::lexer::LoggedToken;

pub enum ExprAST {
  Number(f64),                                      // Numeric literals (floating point value)
  Variable(String),                                 // Variable names (identifier string)
  Binary(LoggedToken, Box<ExprAST>, Box<ExprAST>),  // Binary operator between two expressions (left, right)
  Call(String, Vec<ExprAST>),                       // Function call (function name, argument list)
  Function(String, Vec<ExprAST>, Vec<ExprAST>)      // Function definition (function name, list of identifiers/argument, a list of expressions for the body)
}

pub struct Parser {
  pub tokens: Vec<LoggedToken>,
}

impl Parser {
  pub fn parse(&self) -> Result<Vec<ExprAST>, String> {
    todo!()
  }

  pub fn parseExpr(&self) -> Result<ExprAST, String> {
    let LHS = self.parsePrimaryExpr()?;
    // // Parse any expression (including both the primary ones and bin-ops)
    // auto LHS = parsePrimaryExpr();
  
    // if (!LHS)
    //   return nullptr;
    
    // auto expr = parseBinaryExpr(0, std::move(LHS));
    // return expr;
    todo!()
  }

  pub fn parsePrimaryExpr(&self) -> Result<ExprAST, String> {
    todo!()
  }
}

// std::unique_ptr<AST::Expr> Parser::parsePrimaryExpr() {
//   // Parse basic, not bin-op expressions
//   switch (peek().value().type) {
//     default:
//       return nullptr; // Todo: Throw an error
//     case DecafScanning::TokenType::IDENTIFIER:
//       return identifierExpr();
//     case DecafScanning::TokenType::NUMBER:
//       return numberExpr();
//     case DecafScanning::TokenType::OPEN_PAREN:
//       return groupingExpr();
//     case DecafScanning::TokenType::IF:
//       return conditionalExpr();
//     case DecafScanning::TokenType::WHILE:
//       return whileExpr();
//   }
// }
