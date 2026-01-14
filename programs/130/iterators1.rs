// Создание собственных итераторов с помощью типажа Iterator

//Пример: создадние итератора, который будет всегда считать только от 1 до 5

struct Counter {
  count: u32,
}

impl Counter {
  fn new() -> Counter {
    Counter { count: 0}
  }
}

impl Iterator for Counter {
  // итератор будет возвращать значения типа u32.
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    self.count += 1;

    if self.count < 6 {
      Some(self.count)
    } else {
      None
    }
  }
}

// После того как реализован типаж Iterator, у Сounter появился итератор
#[test]
fn calling_next_directly() {
  let mut counter = Counter::new();

  assert_eq!(counter.next(), Some(1));
  assert_eq!(counter.next(), Some(2));
  assert_eq!(counter.next(), Some(3));
  assert_eq!(counter.next(), Some(4));
  assert_eq!(counter.next(), Some(5));
  assert_eq!(counter.next(), None);
}

// Использование других методов типажа Iterator
/*  Например, хотим значения, произведенные экземпляром структуры Counter:
- zip: соединить их в пары со значениями, произве­
денными еще одним экземпляром структуры Counter 
- skip: после пропуска первого зна­чения
- map: перемножить пары между собой 
- filter: сохранить только те результаты, которые
делятся на 3
- sum: сложить все полученные значения вместе
*/
#[test]
fn using_other_iterator_trait_methods() {
  let sum: u32 = Counter::new()
    .zip(Counter::new()skip(1))
    .map(|(a,b)| a*b)
    .filter(|x| x % 3 == 0)
    .sum();

  assert_eq!(18, sum)
}
