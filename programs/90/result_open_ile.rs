// 3. Открытие файла\
use std::fs::File;

fn main() {
  let f = File::open("hello.txt");

  let f = match f {
    Ok(file) => file,
    Err(error) => {
      panic!("Проблема с открытием файла: {:?}", error)
    },
  };
}