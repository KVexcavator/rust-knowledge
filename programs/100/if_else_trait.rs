// Условная реализация методов для обобщенного типа в зависимости от границ типажа
// Например, тип Pair<T> всегда реализует функцию new. Но Pair<T> реализует метод  cmp_display только в том случае, если его внутренний тип T реализует типаж PartialOrd, позволяющий сравнивать, и типаж Display, позволяющий печатать.

use std::fmt::Display;

struct Pair<T> {
  x: T,
  y: T,
}

impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self {x, y}
  }
}

impl<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self){
    if self.x >= self.y {
      println!("Наиболший член х равен {}", self.x);
    } else {
      println!("Наиболший член y равен {}", self.y);
    }
  }
}