// Выполнение кода при очистке с помощью типажа Drop
// Drop, который индивидуально настраивать то, что происходит, когда значение вот-вот вый­дет из области видимости

// Структура CustomSmartPointer, реализующая типаж Drop там, где мы разместим код очистки

struct CustomSmartPointer {
  data: String,
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    // любая логика когда экземпляр типа выходит из области видимости
    println!("Отбрасывается CustomSmartPointer с данными `{}`!", self.data);
  }
}

fn main(){
  let c = CustomSmartPointer {data: String::from("Мои вещи")};
  let d = CustomSmartPointer {data: String::from("Чужие вещи")};
  println!("Экземпляры CustomSmartPointer созданы.");
}
// Экземпляры CustomSmartPointer созданы.
// Отбрасывается CustomSmartPointer с данными `чужие вещи`!
// Отбрасывается CustomSmartPointer с данными `мои вещи`!