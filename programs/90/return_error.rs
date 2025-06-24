// 9.6. Функция, которая возвращает ошибки в вызывающий код с помощью выражения match
use std::io;
use std::io::Read;
use std::fs;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
  let f = File::open("hello.txt");

  let mut f = match f {
    Ok(file) => file,
    Err(e) => return Err(e),
  };

  let mut s = String::new();

  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e)
  }
}

// Функция, которая возвращает ошибки в вызывающий код с помощью оператора ?
// - ? заменяет match


fn read_username_from_file() -> Result<String, io::Error> {
  let mut f = File::open("hello.txt")?;
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)
}

// ещё короче
fn read_username_from_file() -> Result<String, io::Error> {
  let mut s = String::new();
  File::open("hello.txt")?.read_to_string(&mut s);
  Ok(s)
}

// ещё короче
// Rust предоставляет удобную функцию fs::read_to_string, которая от­крывает файл, создает новый экземпляр типа String, считывает содержимое файла, помещает его содержимое в этот экземпляр и возвращает его.

fn read_username_from_file() -> Result<String, io::Error> {
  fs::read_to_string("hello.txt");
}

// Оператор ? может использоваться только в функциях, возвращающих Result
// из main ещже можно
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
  let f = File::open("hello.txt")?;
  Ok(())
}