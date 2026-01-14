// Наличие нескольких владельцев изменяемых данных путем сочетания Rc<T> и RefCell<T>

// Если есть умный указатель Rc<T>, содержащий RefCell<T>, то можно получить значение, способное иметь нескольких владельцев, которое можно изменять

// Использование умного указателя Rc<RefCell<i32>> для создания экземпляра перечисления List, который мы можем изменять

#[derive(Debug)]
enum List {
  Cons(Rc<RefCell<i32>>, Rc<List>),
  Nil,
}
use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;
fn main() {
  let value = Rc::new(RefCell::new(5));
  let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
  let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
  let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
  *value.borrow_mut() += 10;

  println!("a после = {:?}", a);
  println!("b после = {:?}", b);
  println!("c после = {:?}", c);
}
