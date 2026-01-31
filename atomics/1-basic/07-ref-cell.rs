// RefCell
// std::cell::RefCell 
// - позволяет заимствовать её содержимое
// - содержит не только T, но и счётчик, который отслеживает все незавершённые заимствования
// - может использоваться только в одном потоке

// Заимствование содержимого RefCell осуществляется путём вызова borrow или borrow_mut:
use std::cell::RefCell;
fn f(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(1); // We can modify the `Vec` directly.
}