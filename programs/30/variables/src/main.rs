fn main() {
    let mut x = 5;
    println!("Значение х равно {}", x);
    x = 6;
    println!("Значение х равно {}", x);

    // затемнение
    let y = 5;
    let y = y + 1;
    let y = y * 1;
    println!("Значение y равно {}", y);

    // Типы с плавающей точкой
    let a = 2.0; // f64
    let b: f32 = 3.0; // f32

    // Числовые операции
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    // Булев тип
    let t = true;
    let f: bool = false;

    // Символьный тип char
    let c = 'z';
    let z = 'ƶ';
    let heart_eyed_cat = '😻';

    // Кортежный тип
    let tup = (500, 6.4, 1);
    let (first, second, third) = tup;
    println!("Значение second равно {}", second);
    //с помощью точ­ки (.) и индекса
    let bomba: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = bomba.0;
    let six_point_four = bomba.1
    let one = bomba.2

    // Массив
    let arr_first = [1, 2, 3, 4, 5];
    let arr_second: [i32; 5] = [1, 2, 3, 4, 5];
}
