//Трактовка умных указателей как обыкновенных ссылок с помощью типажа Deref

// Пример с обыкновенной ссылкой
fn main() {
  let x = 5;
  let y = &x;
  assert_eq!(5, x);
  assert_eq!(5, *y); // -*y, чтобы пройти по ссылке к значению
}

//Использование Box<T> в качестве ссылки
fn main() {
  let x = 5;
  let y = Box::new(x);
  assert_eq!(5, x);
  assert_eq!(5, *y);
}

//Определение собственного умного указателя
// Пример построения умного указателя, подобного типу Box<T>
// Тип MyBox представляет собой кортежную структуру с одним элементом типа T
struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}
// Попытка использовать MyBox<T> таким же образом, как ссылки и умный указатель Box<T>
fn main() {
  let x = 5;
  let y = MyBox::new(x);
  assert_eq!(5, x);
  assert_eq!(5, *y);
}
// error[E0614]: type `MyBox<{integer}>` cannot be dereferenced

// Трактовка типа как ссылки путем реализации типажа Deref
// Типаж Deref, требует реализации одного метода с именем deref, который заимствует self и возвращает ссылку на внутренние данные.
// Реализация типажа Deref в типе MyBox<T>
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}
impl<T> Deref for MyBox<T> {
  // связанный тип, который используется типажом Deref
  type Target = T;

  fn deref(&self) -> {
    // deref возвращает ссылку на значение(по оператору *)
    &self.0
  }
}

fn main() {
  let x = 5;
  let y = MyBox::new(x);
  assert_eq!(5, x);
  assert_eq!(5, *y); // - *y фактически выполнил *(y.deref())
}
// Код выше для MyBox заработает
