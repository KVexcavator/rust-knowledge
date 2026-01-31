// В примере используется тип AtomicBool для флага остановки. Такой флаг используется для информирования других потоков о необходимости остановить выполнение

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;

fn main() {
  static STOP: AtomicBool = AtomicBool::new(false);

  // порождаем поток, который делает какуюто работу
  let background_thread = thread::spawn(||{
    while !STOP.load(Relaxed){
      some_work();
    }
  })

  // используем main поток для прослушивания ввода пользователя
  for line in std::io::stdin().lines() {
    match line.unwrap().as_str() {
      "help" => println!("commands: help, stop"),
      "stop" => break,
      cmd => println!("unknown command: {cmd:?}"),
    }
  }

  // cooбщение background_thread, что его необходимо остановить
  STOP.store(true, Relaxed);

  // ожидаем пока background_thread остановится
  background_thread.join().unwrap();
}

// В этом примере background поток многократно выполняет some_work(), в то время как main поток позволяет пользователю вводить команды для взаимодействия с программой
// Когда main поток считывает команду stop, он устанавливает флаг в true, который проверяется background потоком перед каждой новой итерацией
// Main поток ждет, пока фоновый поток завершит свою текущую итерацию, используя метод join
// Если background застревает в some_work() на длительное время, это может привести к недопустимой задержке между командой stop и завершением программы