// 7.1. Модуль front_of_house, содержащий другие модули, которые затем содержат функции


pub mod hosting;
mod serving {
    fn take_order(){}
    fn serve_order(){}
    fn take_payment(){}
}
