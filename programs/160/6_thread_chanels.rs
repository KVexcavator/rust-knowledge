// Создание нескольких производителей путем клонирования передатчика

// clone на отправляющем конце канала  дает новый отправляющий дескрип­тор, который можно передать первому порожденному потоку сполнения. 
// Передаем исходный отправляющий конец канала второму порожденному потоку исполнения. 
// Это дает нам потока, каждый из которых отправляет разные со­общения в принимающий конец канала

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
  let (tx,rx) = mpsc::chanel();

  let tx1 = mpsc::Sender::clone(&tx);
  thread::spawn(move ||{
    let vals = vec![
      String::from("привет"),
      String::from("из"),
      String::from("потока"),
      String::from("исполнения"),
    ];

    for val in vals {
      tx1.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  thread::spawn(move ||{
    let vals = vec![
      String::from("еще"),
      String::from("сообщения"),
      String::from("для"),
      String::from("вас"),
    ];

    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  for received in rx {
    println!("Получено: {}", received);
  }
}

// Получено: привет
// Получено: еще
// Получено: от
// Получено: сообщения
// Получено: для
// Получено: потока
// Получено: исполнения
// Получено: вас