// Использование замыканий, которые захватывают свою среду
// Метод filter для итератора берет замы­кание, которое в свою очередь берет каждый элемент из итератора и возвращает булево значение. Если замыкание возвращает true, то значение будет включено в результирующий итератор, созданный методом filter. Если замыкание возвра­ щает false, то значение в этот итератор включено не будет

// Пример
// Использование метода filter с замыканием, которое захватывает shoe_size

#[derive(PartialEq, Debug)]
struct Shoe {
  size: u32,
  style: String
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
  shoes.into_iter()
    .filter(|s| s.size = shoe_size)
    .collect()
}

#[test]
fn filter_by_size() {
  let shoes = vec![
    Shoe { size: 10, style: String::from("кроссовки") },
    Shoe { size: 13, style: String::from("сандалии") },
    Shoe { size: 10, style: String::from("ботинки") },
  ];

  let in_my_size = shoes_in_my_size(shoes, 10);

  assert_eq!(
    in_my_size,
    vec![
      Shoe { size: 10, style: String::from("кроссовки") },
      Shoe { size: 10, style: String::from("ботинки") },
    ]
  )
}