
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  Add,
  Subtract,
  Multiply,
  Divide,
  Caret, // ^
  LeftParen,
  RightParen,
  Num(f64),
  EOF, // end of scanning the entire expression
}

// Правила приоритета операторов определяют порядок обработки арифметического выражения
// DefaultZero < AddSub < MulDiv < Power < Negative
#[derive(Debug, PartialEq, PartialOrd)]
pub enum OperPrec {
  DefaultZero, // приоритет по умолчанию
  AddSub, // если арифметическая операция — сложение
  MulDiv, // если арифметическая операция — умножение
  Power, // если встречается оператор ^
  Negative, // применяемый к - префиксу
}

impl Token {
  pub fn get_oper_prec(&self) -> OperPrec {
    use self::OperPrec::*;
    use self::Token::*;
    
    match *self {
      Add | Subtract => AddSub,
      Multiply | Divide => MulDiv,
      Caret => Power,

      _ => DefaultZero,
    }
  }
}
