mod front_of_house;

pub use crate::front_of_house::hosting;

fn serve_special_order() {}

mod back_of_house {
    fn fix_incorrect_order(){
        cook_special_order();
        super::serve_special_order();
    }
    fn cook_special_order(){}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("персики"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub mod cleaning {
        pub fn water(){}
        pub fn brash(){}
    }
}


// use crate::back_of_house::cleaning;
// self относительный путь для use
use self::back_of_house::cleaning;


pub fn eat_at_restaurant() {
    // абсолютный путь
    // add_to_waitlist определена в той же упаковке, что и функция eat_at_restaurant. Это означает, что мы можем исполь­ зовать ключевое слово crate
    crate::front_of_house::hosting::add_to_waitlist();
    // относительный путь
    // front_of_house определен на том же уровне дерева модулей, что и eat_at_restaurant
    front_of_house::hosting::add_to_waitlist();

    // заказать летом завтрак с ржаным тостом
    let mut meal = back_of_house::Breakfast::summer("ржаной");
    // Изменить мнение о том, какой хлеб бы мы хотели
    meal.toast = String::from("пшеничный");
    println!("Я бы хотел {} тост, пожалуйста", meal.toast);
    // без pub поменять фрукты не получится
    // meal.seasonal_fruit = String::from("черника");

    // from pub enum
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // with crate::back_of_house::cleaning;
    cleaning::brash();
    // with self use
    cleaning::water();
}

