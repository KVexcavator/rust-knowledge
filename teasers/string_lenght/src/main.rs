const HELLO_WORLD: &'static str = "Halló heimur";

fn main() {

    // вернет 13, но во фразе 12 букв(включая пробел)H
    println!("{} is {} characters long.",
        HELLO_WORLD,
        HELLO_WORLD.len()
    );

    // правильный подход к подчету букв
    println!("{} is {} characters long.",
        HELLO_WORLD,
        HELLO_WORLD
            .chars()
            .count()
    );
}
/*
Строки — это всего лишь вектор байтов (u8), представляющий символы Unicode в кодировке UTF-8.
pub struct String {
    vec: Vec<u8>,
}
Rust автоматически переводит строку в UTF-8.
"Halló heimur" состоит из 11 символов ASCII (включая пробел) и одного символа дополнения Latin-1: ó. Для кодирования символов ASCII требуется 1 байт, для латинских дополнений — 2 байта.
*/