/*
assert! в случае когда нужно убедиться, что условие в тесте принимает значение true. 
Если значе­ние является истинным, то макрокоманда assert! ничего не делает, и тест успешен.
Если значение является ложным, то макрокоманда assert! вызывает макрокоманду panic!, которая приводит к провалу теста
 */

#[derive(Debug)]
pub struct Rectangle {
  length: u32,
  width: u32,
}
impl Rectangle {
  pub fn can_hold(&self, other: &Rectangle) -> bool {
    self.length > other.length && self.width > other.width
  }
}

#[cfg(test)]
mod tests {
  // все, что мы определяем во внешнем модуле, доступно этому модулю
  use super::*;

  fn larger_can_hold_smaller(){
    let larger = Rectangle::new(length: 8, width: 7);
    let smaller = Rectangle::new(length: 5, width: 1);

    assert!(larger.can_hold(&smaller));
  }

  fn smaller_cannot_hold_larger(){
    let larger = Rectangle::new(length: 8, width: 7);
    let smaller = Rectangle::new(length: 5, width: 1);

    assert!(!smaller.can_hold(*larger))
  }
}