use std::fs::File;
use std::io::ErrorKind;

fn main() {
  let f = File::open("hello.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
      File::create("hello.txt").unwrap_or_else(|error| {
        panic!("Проблема с созданием файла: {:?}", error);
      })
    } else {
      panic!("Проблема с открытием файла: {:?}", error);
    }
  }); 
}