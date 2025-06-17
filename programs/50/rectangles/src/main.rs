
#[derive(Debug)] // аннотация для печати структуры
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    // печать структуры для отладки
    //println!("rect1 равен {:#?}", rect1);
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60, 
        height: 45,
    };


    println!("Площадь прямоугольника rest1 равна {} квадратным пикселям.", rect1.area() );
    
    println!("Может ли rect1 содержать в себе rect2? {}", rect1.can_hold(&rect2));
    println!("Может ли rect1 содержать в себе rect3? {}", rect1.can_hold(&rect3));
}

