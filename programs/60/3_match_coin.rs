// 6.3. Перечисление и выражение match, которое в качестве паттернов содержит варианты перечисления

enum Coin {
  Penny,
  Nickel, // пятак
  Dime, // 10 центов
  Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    // рукава match
    // => отделяет патерн от исполняемого кода
    Coin::Penny => {
      println!("Монетка на счастье!");
      1
    },
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}