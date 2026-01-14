// Совместное использование Mutex<T> несколькими потоками
// Использование типа Arc<T> для обертывания типа Mutex<T>, чтобы делиться владением между несколькими потоками
// Десять потоков исполнения, каждый из которых увеличивает счетчик, охраняемый умным указателем Mutex<T>
use std::sync::{Mutex, Arc}
use std::thread;

fn main(){
  let counter = Acr::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
      let mut num = counter.lock().unwrap();
      *num += 1;
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  print("Result: {}", counter.lock().unwrap());
}
// Этот код выведет следующее:
// Результат: 10