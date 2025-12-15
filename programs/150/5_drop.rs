// Досрочное отбрасывание значения с помощью std::mem::drop
// для очистки значений дострочно

// Rust не дает вызывать метод drop типажа Drop вручную c.drop(); 
// error[E0040]: explicit use of destructor method

// std::mem::drop для явного отбрасывания значения, прежде чем оно выйдет из области видимости

fn main() {
  let c = CustomSmartPointer {data: String::from("Некие данные")};
  println!("Экземпляр CustomSmartPointer создан.");
  drop(c);
  println!("CustomSmartPointer отброшен до конца функции main.");
}
// CustomSmartPointer created.
// Dropping CustomSmartPointer with data `some data`!
// CustomSmartPointer dropped before the end of main.