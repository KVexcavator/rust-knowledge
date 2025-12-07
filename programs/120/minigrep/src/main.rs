use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    //unwrap_or_else(из библиотеки Result<T,E>) для настройки обработки ошибок
    let config = Config::new(&args).unwrap_or_else(|err| {
        // println! только для стандартного вывода ДАННЫХ (stdout) 
        // erprintln! для вывода ОШИБОК в стандарный вывод ошибок (stderr)
        eprintln!("Проблемма при разборе аргументов: {}",err);
        // немедленно остановит программу
        process::exit(1);
    });

    // функция run исполнится, но может выдать ошибку
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

