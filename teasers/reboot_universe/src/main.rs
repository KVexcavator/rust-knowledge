#[warn(clippy::float_cmp)]
#[allow(clippy::eq_op)]
fn main() {
    if 0.1 + 0.2 == 0.3 {
        println!("Arithmetic still works.");
    } else {
        println!("Please reboot the universe.");
    }
    // правильно
    if (0.1f32 + 0.2f32 - 0.3f32).abs() < f32::EPSILON {
        println!("Arithmetic works");
    } else {
        println!("Please reboot the universe.");
    }
}
/*
вернгёт
Please reboot the universe.

float хранятся с погрешностью, например:
println!("{}", 0.1f32 + 0.2f32); // 0.300000012...

увидеть ошибку с clippy можно настроив 
#[warn(clippy::pedantic)]
убрать ворчание clippy на стиль
#[allow(clippy::eq_op)]
*/
