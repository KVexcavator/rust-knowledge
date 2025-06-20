// 6.5. Функция, использующая выражение match для перечисления Option<i32>

// функция, которая берет перечисление Option<i32> и, если внутри есть значение, то эта функция  прибавляет к нему 1

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);