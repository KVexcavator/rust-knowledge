// Проверка того, что условие станет причиной для panic

pub struct Guess {
  value: i32,
}

impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 || value > 100 {
      panic!("Значение догадки должно быть между 1 и 100, получено {}.", value);
    }
    Guess{
      value
    }
  }
}

// не обязательны параметр expected, проверяет что panic! будет содержать особое сообщение о панике
pub struct GuessExpected{
  value: i32,
}

impl GuessExpected {
  pub fn new(value: i32) -> Guess {
  if value < 1 {
    panic!("Значение догадки должно быть больше или равно 1,   получено {}.", value);
  } else if value > 100 {
    panic!("Значение догадки должно быть меньше или равно 100,   получено {}.", value);
  }
  }
  GuessExpected {
    value
  }
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic]
  fn greater_than_100(){
    Guess::new(200);
  }

  #[test]
    #[should_panic(expected = "Значение догадки должно быть меньше  или равно 100")]
    fn expected_greater_than_100() {
      GuessExpected::new(200);
    }
}