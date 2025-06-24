fn vec_number_simple_sort(numbers: &Vec<i32>) {
  let mut sorted = numbers.clone();
  sorted.sort();
  println!("{:?}", sorted);
}

fn vec_reverse_sort(numbers: &Vec<i32>) {
  let mut sorted = numbers.clone();
  sorted.sort_by(|a, b| b.cmp(a)); // обратное убывание
  println!("{:?}", sorted);
}

#[derive(Debug, Clone)]
struct User {
  name: String,
  age: u32,
}

fn sort_struct_by_age(users: &Vec<User>){
  let mut sorted_users = users.clone();
  sorted_users.sort_by(|a, b| a.age.cmp(&b.age));

  for user in sorted_users {
    println!("{:?}", user);
  }
}

fn main() {
  let numbers = vec![4, 2, 5, 1, 3];
  vec_number_simple_sort(&numbers);
  vec_reverse_sort(&numbers);

  let users = vec![
    User { name: "Alice".into(), age: 30 },
    User { name: "Bob".into(), age: 25 },
    User { name: "Charlie".into(), age: 35 },
  ];

  sort_struct_by_age(&users);

}

// rustc vec_sort.rs
// chmod +x vec_sort.rs
// ./vec_sort.rs