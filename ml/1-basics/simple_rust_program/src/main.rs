extern crate ndarray;
use ndarray::Array;

fn main() {
    let array1 = Array::from_shape_vec((2,2), vec![1., 2., 3., 4.]).unwrap();
    let array2 = Array::from_shape_vec((2,2), vec![4., 3., 2., 1.]).unwrap();

    println!("Array 1:\n{:?}", array1);
    println!("Array 2:\n{:?}", array2);

    let sum = &array1 + &array2;
    let product = &array1 * &array2;

    println!("---");
    println!("Sum of arrays:\n{:?}", sum);
    println!("Element-wise product of arrays:\n{:?}", product);
}
/*
вернет:
Array 1:
[[1.0, 2.0],
 [3.0, 4.0]], shape=[2, 2], strides=[2, 1], layout=Cc (0x5), const ndim=2
Array 2:
[[4.0, 3.0],
 [2.0, 1.0]], shape=[2, 2], strides=[2, 1], layout=Cc (0x5), const ndim=2
 ---
Sum of arrays:
[[5.0, 5.0],
 [5.0, 5.0]], shape=[2, 2], strides=[2, 1], layout=Cc (0x5), const ndim=2
Element-wise product of arrays:
[[4.0, 6.0],
 [6.0, 4.0]], shape=[2, 2], strides=[2, 1], layout=Cc (0x5), const ndim=2
 */
