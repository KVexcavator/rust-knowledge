// Создание абстракции поведения с помощью замыканий
// Пример, создает план тренировок
use std::thread;
use std::time::Duration;




// Функция выводит планы тренировок на основе входных данных и вызывает функцию simulated_expensive_calculation
// intensity указывает, какая тренировка необходима — низкоин­тенсивная или высокоинтенсивная.
fn generate_workout(intensity: u32, random_number: u32) {
  // Определение замыкания и сохранение его в переменной
  let expensive_closure = |num| {
    println!("медленное вычисление");
    thread::sleep(Duration::from_secs(2));
    num
  };
  
  if intensity < 25 {
    println!(
        "Сегодня сделайте {} отжиманий!",
        expensive_closure(intensity)
    )
    println!(
        "Далее, сделайте {} приседаний!",
        expensive_closure(intensity)
    )
  } else {
    if random_number == 3 {
      println!("Сделайте сегодня перерыв! Пейте больше воды!");
    } else {
      println!(
        "Сегодня пробежка {} минут!",
        expensive_closure(intensity)
      );
    }
  }  
}

// Функция main с жестко заданными значениями для моделирования ввода пользователем данных и генерации случайных чисел
fn main() {
  // для простоты примера захаркодено
  // в ральности долго запрашивается и вычисляется
  let simulated_user_specified_value = 10;
  let simulated_random_number = 7;

  generate_workout(
    simulated_user_specified_value,
    simulated_random_number
  )
}