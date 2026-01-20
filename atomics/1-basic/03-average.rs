use std::thread;
use std::iter::FromIterator;

fn main() {
  let numbers = Vec::from_iter(0..=1000);

  let t = thread::spawn(move || {
    let len = numbers.len();
    let sum = numbers.into_iter().sum::<usize>();
    sum / len
  });

  let average = t.join().unwrap();
  println!("Average: {average}");
}
// это не компилируется
// rustc 03-average
// error: couldn't read `03-average`: stream did not contain valid UTF-8
//  --> 03-average:1:26
//   |
// 1 | ␡ELF␂␁␁␀␀␀␀␀␀␀␀␀␃␀>␀␁␀␀␀ �␁␀␀␀␀␀@␀␀␀␀␀␀␀h�<␀␀␀␀␀␀␀␀␀@␀8␀␌␀@␀)␀'␀␆␀␀...
//   |                          ^ byte `155` is not valid utf-8

// error: aborting due to 1 previous error

