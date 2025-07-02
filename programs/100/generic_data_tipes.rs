// Две функции, отличающиеся только именами и типами в сигнатурах

fn largest_i32(list: &[i32]) -> i32 {
  let mut largest = list[0];

  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }
  largest
}

fn largest_char(list: &[char]) -> char {
  let mut largest = list[0];

  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }
  largest
}

fn main() {
  let number_list = vec! [34, 50, 25, 100, 65];
  let result = largest_i32(&number_list);
  println!("Наибольшее число равно {}", result);

  let char_list = vec! ['y', 'm', 'a', 'c'];
  let result = largest_char(&char_list)
  println!("Наибольшее число равно {}", result);
}

// Определение функции largest, которая использует параметры обобщенного типа, но пока не компилируется

fn largest<T>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }
  largest
}

fn main() {
  let number_list = vec! [34, 50, 25, 100, 65];
  let result = largest(&number_list);
  println!("Наибольшее число равно {}", result);

  let char_list = vec! ['y', 'm', 'a', 'c'];
  let result = largest(&char_list)
  println!("Наибольшее число равно {}", result);
}
// этот код выдаст ошибку
// binary operation `>` cannot be applied to type `T`

// ====================================
// другие приемы использования параметров обобщенного типа

// Структура Point<T>, содержащая значения x и y типа T
struct Point<T>  {
  x: T,
  y: T
}
fn main(){
  let integer = Point {x: 5, y: 10};
  let float = Point {x: 1.0, y: 4.0};
}
// let wont_work = Point { x: 5, y: 4.0 }; -> Err

// Point<T, U> является обобщением двух типов, в результате чего x и y могут быть значениями разных типов
struct Point<T,U> {
  x: T,
  y: U,
}
fn main(){
  let both_integer = Point {x: 5, y: 10};
  let both_float = Point {x: 1.0, y: 4.0};
  let integer_and_float = Point {x: 5, y: 4.0};
}

// В определениях перечислений
enum Options<T> {
  Some(T),
  None,
}
// выражает идею необязательного параметра
enum Result<T, E> {
  Ok(T),
  Err(E),
}

// В определениях методов

// Реализация метода x в структуре Point<T>, который будет возвращать ссылку на поле x типа T
struct Point<T> {
  x: T,
  y: T,
}
impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}
fn main(){
  let p = Point {x: 5, y: 10};
  println!("p.x = {}", p.x());
}
// Благодаря объявлению T как обобщенного типа после impl, Rust бу­дет отождествлять тип в угловых скобках в Point как обобщенный, а не конкретный.

// Блок impl, который применим только к структуре с отдельно взятым конкретным типом для параметра обобщенного типа T

impl Point<f32> {
  fn distance_from_original(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}
// Этот код означает, что тип Point<f32> имеет метод distance_from_origin, а для других экземпляров типа Point<T>, где T не относится к типу f32, этот метод опре­делен не будет.

// Метод, который использует другие обобщенные типы, отличные от определенной структуры
struct Point<T,U> {
  x: T,
  y: U,
}
impl<T,U> Point<T,U> {
  fn mixup<V, W>( self, other: Point<V,W>) -> Point<T, W> {
    Point {
      x: self.x,
      y: other.y,
    }
  }
}
fn main(){
  let p1 = Point {x: 5, y: 10.4};
  let p2 = Point {x: "Hello", y: 'c'};

  let p3 = p1.mixup(p2);
}
// ================================