//! Парсер — это модуль, который строит дерево узлов(AST), где каждый узел представляет собой токен (число или арифметический оператор)

use std::fmt;
use super::ast::Node;
use super::token::{OperPrec, Token};
use super::tokenizer::Tokenizer;

pub struct Parser<'a> {
  tokenizer: Tokenizer<'a>,
  current_token: Token,
}

// публичные методы Parser
impl<'a> Parser<'a> {
  // создаёт новый экземпляр Tokenizer, инициализируя его арифметическим выражением, а затем пытается извлечь первый токен из выражения. В случае успеха токен сохраняется в поле current_token или ParseError.
  pub fn new(expr: &'a str) -> Result<Self, ParseError> {
    let mut lexer = Tokenizer::new(expr);
    let cur_token = match lexer.next() {
      Some(token) => token,
      None => return Err(ParseError::InvalidOperator("Invalid character".into()))
    };
    Ok(Parser {
      tokenizer: lexer,
      current_token: cur_token,
    })
  }

  // вызывает приватный метод generate_ast(), который выполняет рекурсивную обработку и возвращает AST (дерево узлов). В случае успеха он возвращает дерево узлов; в противном случае он распространяет полученную ошибку
  pub fn parse(&mut self) -> Result<Node, ParseError> {
    let ast = self.generate_ast(OperPrec::DefaultZero);
    match ast {
      Ok(ast) => Ok(ast),
      Err(e) => Err(e),
    }
  }
}

// Приватныe методы Parser
impl<'a> Parser<'a> {
  // извлекает следующий токен из арифметического выражения, используя структуру Tokenizer, и обновляет поле current_token структуры Parser
  fn get_next_token(&mut self) -> Result<(), ParseError> {
    let next_token = match self.tokenizer.next() {
      Some(token) => token,
      None => return Err(ParseError::InvalidOperator("Invalid character".into())),
    };
    self.current_token = next_token;
    Ok(())
  }
  // для проверки наличия пар скобок в выражении
  fn check_paren(&mut self, expected: Token) -> Result<(), ParseError> {
    if expected == self.current_token {
      self.get_next_token()?;
      Ok(())
    } else {
      Err(ParseError::InvalidOperator(format!("Expected {:?}, got {:?}", expected, self.current_token)))
    }
  }
  //Является ли токен числом вида Num(i)
  //Имеет ли токен знак, если это отрицательное число. Например, выражение -2,2 + 3,4 анализируется в AST как Add(Negative(Number(2,2)),Number(3,4))
  // Пары скобок: Если выражение находится внутри пар скобок, оно обрабатывается как операция умножения. Например, 1*(2+3) анализируется как Multiply(Number(1,0), Add(Number(2,0), Number(3,0)))
  fn parse_number(&mut self) -> Result<Node, ParseError> {
    let token = self.current_token.clone();
    match token {
      Token::Subtract => {
        self.get_next_token()?;
        let expr = self.generate_ast(OperPrec::Negative)?;
        Ok(Node::Negative(Box::new(expr)))
      }
      Token::Num(i) => {
        self.get_next_token()?;
        Ok(Node::Number(i))
      }
      Token::LeftParen => {
        self.get_next_token()?;
        let expr = self.generate_ast(OperPrec::DefaultZero)?;
        self.check_paren(Token::RightParen)?;
        if self.current_token == Token::LeftParen {
          let right = self.generate_ast(OperPrec::MulDiv)?;
          return Ok(Node::Multiply(Box::new(expr), Box::new(right)));
        }
        Ok(expr)
      }
      _ => Err(ParseError::UnableToParse("Unable to parse".to_string())),
    }
  }
  // обрабатывает числовые токены, токены отрицательных чисел и выражения в скобках с помощью метода parse_number()
  // анализирует каждый токен арифметического выражения последовательно в цикле, чтобы проверить приоритет следующих двух встречающихся операторов, и конструирует AST, вызывая метод convert_token_to_node() таким образом, что выражение, содержащее оператор с более высоким приоритетом, выполняется раньше выражения, содержащего оператор с более низким приоритетом.
  // Например, выражение 1+2*3 вычисляется как Add(Number(1.0), Multiply(Number(2.0), Number(3.0))), тогда как выражение 1*2+3 вычисляется как Add(Multiply(Number(1.0), Number(2.0)), Number(3.0))
  fn generate_ast (&mut self, oper_prec: OperPrec) -> Result<Node, ParseError> {
    let mut left_expr = self.parse_number()?;

    while oper_prec < self.current_token.get_oper_prec() {
      if self.current_token == Token::EOF {
        break;
      }
      let right_expr = self.convert_token_to_node(left_expr.clone())?;
      left_expr = right_expr;
    }
    Ok(left_expr)
  }

  // метод конструирует узлы AST-типа оператора, проверяя, является ли токен Add, Subtract, Multiply, Divide или Curet. В случае ошибки ParseError
  fn convert_token_to_node(&mut self, left_expr: Node) -> Result<Node, ParseError> {
    match self.current_token {
      Token::Add => {
        self.get_next_token()?;
        let right_expr = self.generate_ast(OperPrec::AddSub)?;
        Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
      }
      Token::Subtract => {
        self.get_next_token()?;
        let right_expr = self.generate_ast(OperPrec::AddSub)?;
        Ok(Node::Subtract(Box::new(left_expr), Box::new(right_expr)))
      }
      Token::Multiply => {
        self.get_next_token()?;
        let right_expr = self.generate_ast(OperPrec::MulDiv)?;
        Ok(Node::Multiply(Box::new(left_expr), Box::new(right_expr)))
      }
      Token::Divide => {
        self.get_next_token()?;
        let right_expr = self.generate_ast(OperPrec::MulDiv)?;
        Ok(Node::Divide(Box::new(left_expr), Box::new(right_expr)))
      }
      Token::Caret => {
        self.get_next_token()?;
        let right_expr = self.generate_ast(OperPrec::Power)?;
        Ok(Node::Caret(Box::new(left_expr), Box::new(right_expr)))
      }
      _ => Err(ParseError::InvalidOperator(format!("Please enter valid operator {:?}", self.current_token))),
    }
  }

}

// Обработчик ошибок
// UnableToParse при общей ошибке для любого типа ошибки во время обработки
// InvalidOperator , если пользователь указал недопустимый арифметический оператор; например, 2=3
#[derive(Debug)]
pub enum ParseError {
  UnableToParse(String),
  InvalidOperator(String),
}

// для автоматического преобразования любой упакованной ошибки из модуля AST в ParseError
impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &self {
      self::ParseError::UnableToParse(e) => write!(f, "Error in evaluating {}", e),
      self::ParseError::InvalidOperator(e) => write!(f, "Error in evaluating {}", e),
    }
  }
}

impl std::convert::From<std::boxed::Box<dyn std::error::Error>> for ParseError {
    fn from(_evalerr: std::boxed::Box<dyn std::error::Error>) -> Self {
        return ParseError::UnableToParse("Unable to parse".into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsemath::ast::Node::{Add, Number};
    #[test]
    fn test_addition() {
        let mut parser = Parser::new("1+2").unwrap();
        let expected = Add(Box::new(Number(1.0)), Box::new(Number(2.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
}
