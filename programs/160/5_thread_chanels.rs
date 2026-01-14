// Отправка нескольких значений и ожидание приемника

// Отправка нескольких сообщений и пауза между ними
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
  let (tx,rx) = mpsc::chanel();

  thread::spawn(move ||{
    let vals = vec![
      String::from("привет"),
      String::from("из"),
      String::from("потока"),
      String::from("исполнения"),
    ];

    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  // не вызываем функцию recv явно: вме­сто этого мы трактуем rx как итератор.
  for received in rx {
    println!("Получено: {}", received);
  }
}

// Получено: привет
// Получено: из
// Получено: потока
// Получено: исполнения