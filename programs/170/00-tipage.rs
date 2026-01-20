// Определение типажа Draw
// src/lib.rs
pub trait Draw {
  fn draw(&self);
}
// Определение структуры Screen с полем components, содержащим вектор типажных объектов, реализующих типаж Draw
// src/lib.rs
pub struct Screen {
  pub components: Vec<Box<dyn Draw>>,
}
// Метод run в структуре Screen, который вызывает метод draw для каждого компонента
//src/lib.rs
impl Screen {
  pub fn run(&self){
    for component im self.components.iter(){
      component.draw();
    }
  }
}
// Альтернативная реализация структуры Screen и ее метода run с использованием обобщений и границ типажа
//src/lib.rs
pub struct Screen<T: Draw> {
  pub fn run(&self){
    for component in self.components.iter(){
      component.draw();
    }
  }
}
// Структура Button, реализующая типаж Draw
// src/lib.rs
pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String,
}

impl Draw for Button {
  fn draw(&self){
    // here code to add button
  }
}

// Еще одна упаковка, использующая gui и реализующая типаж Draw в структуре SelectBox
// src/main.rs
use gui::Draw;

struct SelectBox {
  width:: u32,
  height: u32,
  options: Vec<String>,
}

impl Draw for SelectBox {
  fn draw(&self) {
    // code...
  }
}

// Использование типажных объектов для хранения значений разных типов, реализующих один и тот же типаж
// src/main.rs
use gui::{Screen, Button};

fn main(){
  let screen = Screen {
    components: vec![
      Box::new(SelectBox {
        width: 75,
        height: 10,
        options: vec![
          String::from("Yes"),
          String::from("Maybe"),
          String::from("No")
        ],
      }),
      Box::new(Button {
        width: 50,
        height: 10,
        label: String::from("Ok"),
      }),
    ],
  };

  screen.run();
}