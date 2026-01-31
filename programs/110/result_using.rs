// Использование типа Result<T, E> в тестах

#[cfg(test)]
mod tests {

  #[test]
  fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
      Ok(())
    } else {
      Err(String::from("два плюс два не равно четырем"))
    }
  }
}