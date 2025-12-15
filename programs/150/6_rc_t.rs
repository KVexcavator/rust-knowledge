// Применение Rc<T> для совместного использования данных

// Нельзя иметь два списка, использующих умный указатель Box<T>, которые пытаются совместно владеть третьим списком
// см Cons 1_box_t.rs
enum List {
  Cons(i32, Box<List>),
  Nil,
}

use crate::List::{Cons, Nil};

fn main() {
  let a = Cons(5,
    Box::new(Cons(10,
      Box::new(Nil))));
  // оба списка будут совместно использовать первый список
  let b = Cons(3, Box::new(a));
  let c = Cons(4, Box::new(a)):
}
// error[E0382]: use of moved value: `a`
// ошибка[E0382]: использование перемещенного значения `a`

// Определение типа List, который использует умный указатель Rc<T>
enum List {
  Cons(i32, Rc<List>),
  Nil,
}
use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
  let a = Rc::new(Cons(5,
    Rc::new(10,
      Rc::new(Nil))));
  let b = Cons(3, Rc::clone(&a));
  let c = Cons(4, Rc::clone(&a));
}