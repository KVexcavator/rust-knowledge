use std::error;

// Список разрешённых узлов AST, которые может создать Parser
// Токены могут быть арифметическими операторами или числами
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
  Add(Box<Node>, Box<Node>),
  Subtract(Box<Node>, Box<Node>),
  Multiply(Box<Node>, Box<Node>),
  Divide(Box<Node>, Box<Node>),
  Caret(Box<Node>, Box<Node>),
  Negative(Box<Node>),
  Number(f64),
}

//функция оценщика(evaluator func) рекурсивно анализирует каждый узел в дереве AST и приходит к конечному значению.
// Например, если узел AST — Add(Number(1.0),Number(2.0)), его значение равно 3.0
/*Если узел AST — Add(Number(1.0),Multiply(Number(2.0),Number(3.0)):
- значение Number(1.0) вычисляется как 1.0.
- затем значение Multiply(Number(2.0),Number(3.0)) вычисляется как 6.0.
- затем значение складывается с 1.0 и 6.0, чтобы получить конечное значение 7.0.
*/
pub fn eval(expr: Node) -> Result<f64, Box<dyn error::Error>> {
  use self::Node::*;
  match expr {
    Number(i) => Ok(i),
    Add(expr1, expr2) => Ok(eval(*expr1)? + eval(*expr2)?),
    Subtract(expr1, expr2) => Ok(eval(*expr1)? - eval(*expr2)?),
    Multiply(expr1, expr2) => Ok(eval(*expr1)? * eval(*expr2)?),
    Divide(expr1, expr2) => Ok(eval(*expr1)? / eval(*expr2)?),
    Negative(expr1) => Ok(-(eval(*expr1)?)),
    Caret(expr1, expr2) => Ok(eval(*expr1)?.powf(eval(*expr2)?)),
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_expr1() {
        use crate::parsemath::parser::Parser;

        let ast = Parser::new("1+2-3").unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 0.0);
    }
    #[test]
    fn test_expr2() {
        use crate::parsemath::parser::Parser;

        let ast = Parser::new("3+2-1*5/4").unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 3.75);
    }
}

