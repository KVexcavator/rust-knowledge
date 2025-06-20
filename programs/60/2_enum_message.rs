// Перечисление Message, в каждом из вариантов которого хранятся разные объемы и типы значений

// вариант со стуктурами
struct QuitMessage: // пустая структура
struct MoveMessage{
  x: i32,
  y: i32,
}
struct WriteMessage(String) // картежная структура
struct ChangeColorMessage(i32, i32, i32) // картежная структура

// то же самое с enum
enum Message {
  Quit,
  Move {x: i32, y: i32}, // анонимная структура
  Write(String),
  ChangeColor(i32, i32, i32),
}

// для enum(как и для struct) можно определить методы

impl Message {
  fn call(&self){}
}
let m = Message::Write(String::from("hello"));
m.call()