fn main() {
    branches_if();
    many_if();
    let_and_if();
}

// Выражения if
fn branches_if() {
    let number = 3;

    // здесь выражение должно быть булевым
    if number < 5 {
        println!("условие было истинным");
    } else {
        println!("условие быо ложным");
    }
}

// Обработка нескольких условий с помощью else if
fn many_if() {
    let number = 6;

    if number % 4 == 0 {
        println!("число делится на 4");
    } else if number % 3 == 0 {
        println!("число делится на 3");
    } else if number % 2 == 0 {
        println!("число делится на 2");
    } else {
        println!("число не делится на 4, 3 и 2");
    }
}

// Использование выражения if в инструкции let
fn let_and_if() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("Number is {}", number)
}
