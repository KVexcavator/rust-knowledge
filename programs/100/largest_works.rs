// Исправление функции largest с помощью границ типажа

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }
  largest
}

fn main() {
  let number_list = vec! [34, 50, 25, 100, 65];
  let result = largest(&number_list);
  println!("Наибольшее число равно {}", result);

  let char_list = vec! ['y', 'm', 'a', 'c'];
  let result = largest(&char_list)
  println!("Наибольшее число равно {}", result);
}