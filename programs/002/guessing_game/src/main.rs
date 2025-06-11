use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Угадайте число!");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Пожалуйстаб введите Вашу догадку.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Не получилось прочитать строку");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Вы загадали: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком малое число!"),
            Ordering::Greater => println!("Сишком большее число!"),
            Ordering::Equal => {
                println!("Вы выиграли");
                break;
            }
        }
    }
}
