use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // env::args возвращает итератор
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Проблемма при разборе аргументов: {}",err);
        process::exit(1);
    });

    
    if let Err(e) = minigrep::run(config) {
        eprintln!("Ошибка в приложении: {}", e);
        process::exit(1);
    }
}

// cargo run the poem.txt
// cargo run frog poem.txt
// cargo run body poem.txt
// пробуем найти то чего нет
// cargo run monomorphization poem.txt

// добавить переменную
// export CASE_INSENSITIVE=1
// удалить 
// unset CASE_INSENSITIVE
// проверить
// echo $CASE_INSENSITIVE
// проверка что переменная работает для переключения поиска
// cargo run to poem.txt - должно вернуть разное

// данные с println выводятся в output.txt, а ошибки с eprintln выводятся в консоль
// $ cargo run > output.txt

