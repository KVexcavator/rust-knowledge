/*
assert_eq! завершится успешно, если два значения равны
assert_ne! завершится успешно, если два значения не равны
Неявно обе макрокоманды — assert_eq! и assert_ne! — используют операторы == и != соответственно
 */
pub fn add_two(a: i32) -> i32 {
  a + 2
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_adds_two() {
    assert_eq!(4, add_two(2));
  }
}