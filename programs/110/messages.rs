/*
Добавление сообщений об ошибках для пользователя
Любые аргументы, которые указываются по­ сле одного обязательного аргумента для assert! или двух обязательных аргументов для assert_eq! и assert_ne!, передаются вместе с макрокомандой format!
 */

pub fn greeting(name: &str) -> String {
  format!("Здравствуй {}!", name)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn greeting_contains_name() {
    let result = greeting("Кэрол");
    assert!(
      result.contains("Кэрол"),
      // сообщение будет выведено при падении теста
      "Приветствие не содержало имя, предоставлено значение `{}`", result
    );
  }
}