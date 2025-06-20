// полезным свойством рукавов выражения match является то, что они могут привязываться к частям значений, которые совпадают с паттерном

// Перечисление Coin, в котором вариант Quarter также имеет значение UsState

#[derive(Debug)] // проверит штат сразу
enum UsState {
  Alabama,
  Alaska,
  // ...
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("Чертвертак из штата {:?}!", state);
      25
    },
  }
}

value_in_cents(Coin::Quarter(UsState::Alaska));