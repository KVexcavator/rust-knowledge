fn main() {
    another_function(5, 6);
    instruction();
    // значение из функции
    let a = five();
    println!("Значение a равно {}", a);
    let b = plus_one(5);
    println!("Значение b равно {}", b);
}

fn another_function(x: i32, y: i32) {
    println!("Значение х равно {}", x);
    println!("Значение y равно {}", y);
}

fn instruction() {
    let x = 5; // это инструкция, ничегоне возвращает

    // блок соозадет новую область видимости
    let y = { 
        let x = 3;
        x + 1
    };
    println!("Значение y pавно {}", y);
}

// Функции с возвращаемыми значениями
fn five() -> i32 {
    5 // выражение
}
fn plus_one(x: i32) -> i32 {
    x + 1 // eсли поставить (;) та станет инструкцие и произойдет ошибка
}