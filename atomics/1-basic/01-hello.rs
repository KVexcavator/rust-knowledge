// Threads in Rust
// Каждая программа начинается ровно с одного потока: основного потока.
// Новые потоки создаются с помощью std::thread::spawn.

use std::thread;

fn main() {
  let t1 = thread::spawn(f);
  let t2 = thread::spawn(f);

  println!("Hello from the main thread.");

  // join гарантарует что потоки завершаться 
  t1.join().unwrap();
  t2.join().unwrap();
}

fn f(){
  println!("Hello from another thread.");

  let id = thread::current().id();
  println!("This is my thread id: {id:?}");
}

// Hello from the main thread.
// Hello from another thread.
// This is my thread id: ThreadId(2)
// Hello from another thread.
// This is my thread id: ThreadId(3)