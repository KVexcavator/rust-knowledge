// Код для поиска наибольшего числа в списке чисел

fn main(){
  let number_list = vec! [34, 50, 25, 100, 65];
  let mut largest = number_list[0];
  for number in number_list {
    if number > largest {
      largest = number;
    }
  }
  println!("Наибольшее число равно {}", largest);
}

// Код для поиска наибольшего числа в двух списках чисел


fn main(){
  let number_list = vec! [34, 50, 25, 100, 65];
  let mut largest = number_list[0];
  for number in number_list {
    if number > largest {
      largest = number;
    }
  }
  println!("Наибольшее число равно {}", largest);

  let number_list = vec! [102, 34, 6000, 69, 65, 8];
  let mut largest = number_list[0];
  for number in number_list {
    if number > largest {
      largest = number;
    }
  }
  println!("Наибольшее число равно {}", largest);
}
// это работает но код ужасный

// Абстрактный код для поиска наибольшего числа в двух списках

fn largest(list: &[i32]) -> i32 {
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

  let number_list = vec! [102, 34, 6000, 69, 65, 8];
  let result = largest(&number_list)
  println!("Наибольшее число равно {}", result);
}
