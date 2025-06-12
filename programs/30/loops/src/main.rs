fn main() {
    loop_example();
    while_example();
    while_iter();
    for_example();
    for_with_range();
}

// Loop
fn loop_example(){
    let mut counter = 0;

    let result = loop {
        counter +=1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Результат равен {}", result);
}

// while
fn while_example() {
    let mut number = 3;

    while number !=0 {
        println!("{}", number);
        number = number - 1;
    }
    println!("Let's go!!!");
}
// если ошибиться с индексом будет паника
fn while_iter() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("Значение равно {}", a[index]);
        index = index + 1;
    }
}

// for
fn for_example() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("Значение равно {}", element);
    }
}
// rev — выводить наоборот
// (1..4) не включать 4, (1..=4) включать 4
fn for_with_range() {
    for number in (1..=4).rev() {
        println!("{}", number);
    }
    println!("Let's Go!!!");
}
