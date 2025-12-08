// Хранение замыканий с использованием обобщенных параметров и типажей Fn
// Мы можем создать структуру, которая будет содержать замыкание и результирующее значение вызова замыкания. Она будет кэшировать результирующее значение, мемоизация.
// Все замыкания реализуют по крайней мере один из трех типажей: Fn, FnMut или FnOnce.

//Определение структуры Cacher, содержащей замыкание в calculation и необязательный результат в value
struct Cacher<T> 
  where T: Fn(u32) -> u32
{
  calculation: T,
  value: Option<u32>,
}

//Алгоритм кэширования структуры Cacher
impl<T> Cacher<T>
  // тот же типаж, что и структура Cacher
  where T: fn(u32) -> u32
{ 
  // берет обобщенный параметр T
  fn new(calculation: T) -> Cacher<T> {
    // возвращает эк­земпляр структуры Cacher
    Cacher {
      calculation,
      value: None
    }
  }

  // метод проверяет, есть ли у нас результирующее значение в self.value внутри Some
  fn value(&mut self, arg: u32) -> u32 {
    main self.value {
      // Если self.value есть, возвращает значение внутри Some, не исполняя замыкание снова
      Some(v) => v,
      // Если self.value равно None, то код вызывает замыкание, хранящееся в self.calculation, сохраняет результат self.value для будущего использования, а так­ же возвращает значение
      None => {
        let v = (self.calculation)(arg);
        self.value = Some(v),
        v
      }
    }
  }
}

// Использование структуры Cacher в функции generate_workout, чтобы абстрагировать алгоритм кэширования
fn generate_workout(intensity: u32, random_number: u32) {
  // Определение замыкания и сохранение его в переменной
  let mut expensive_result = Cacher::new(|num| {
    println!("медленное вычисление");
    thread::sleep(Duration::from_secs(2));
    num
  });
  
  if intensity < 25 {
    println!(
        "Сегодня сделайте {} отжиманий!",
        expensive_result.value(intensity)
    )
    println!(
        "Далее, сделайте {} приседаний!",
        expensive_result.value(intensity)
    )
  } else {
    if random_number == 3 {
      println!("Сделайте сегодня перерыв! Пейте больше воды!");
    } else {
      println!(
        "Сегодня пробежка {} минут!",
        expensive_result.value
      );
    }
  }  
}
